extern crate libc;

use libc::{
    epoll_create1, epoll_ctl, epoll_wait, socket, bind, listen, accept,
    EPOLLIN, EPOLL_CLOEXEC, EPOLL_CTL_ADD, SOCK_STREAM, AF_INET,
    sockaddr, sockaddr_in, sockaddr_in as sockaddr_in_t,
};
use clap::{App, Arg};
use std::fs::File;
use std::io::{self, Read};
use std::mem;
use std::net::{SocketAddr, IpAddr};
use std::path::Path;
use std::ptr;
use std::process::Command;

const MAX_EVENTS: usize = 10;
const CGI_PATH: &str = "/cgi-bin/";

fn main() -> io::Result<()> {
    // Parse command line arguments
    let matches = App::new("localhosh")
        .version("1.0")
        .author("Your Name")
        .about("HTTP server")
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .default_value("8080")
                .takes_value(true)
                .help("Port to listen on"),
        )
        .get_matches();

    let port: u16 = matches
        .value_of_t("port")
        .unwrap_or_else(|e| e.exit());

    // Adresse et port de serveur
    let address = "0.0.0.0";

    unsafe {
        // Crée un descripteur epoll
        let epoll_fd = epoll_create1(EPOLL_CLOEXEC);
        if epoll_fd < 0 {
            return Err(io::Error::last_os_error());
        }

        // Crée un descripteur de socket
        let sockfd = socket(AF_INET, SOCK_STREAM, 0);
        if sockfd < 0 {
            return Err(io::Error::last_os_error());
        }

        // Configure l'adresse du serveur
        let addr = SocketAddr::new(address.parse().unwrap(), port);
        let mut sockaddr: sockaddr_in_t = mem::zeroed();
        sockaddr.sin_family = AF_INET as u16;
        sockaddr.sin_port = port.to_be();
        sockaddr.sin_addr.s_addr = match addr.ip() {
            IpAddr::V4(ipv4) => u32::from_be_bytes(ipv4.octets()),
            IpAddr::V6(_) => panic!("IPv6 not supported"),
        };

        let addr_ptr: *const sockaddr = &sockaddr as *const _ as *const sockaddr;
        if bind(sockfd, addr_ptr, mem::size_of::<sockaddr_in_t>() as u32) < 0 {
            return Err(io::Error::last_os_error());
        }

        if listen(sockfd, 128) < 0 {
            return Err(io::Error::last_os_error());
        }

        // Enregistre le socket auprès de epoll
        let mut event: libc::epoll_event = mem::zeroed();
        event.events = EPOLLIN as u32;
        event.u64 = sockfd as u64;

        if epoll_ctl(epoll_fd, EPOLL_CTL_ADD, sockfd, &mut event as *mut _) < 0 {
            return Err(io::Error::last_os_error());
        }

        let mut events = vec![libc::epoll_event { events: 0, u64: 0 }; MAX_EVENTS];

        loop {
            // Attendre les événements
            let num_events = epoll_wait(epoll_fd, events.as_mut_ptr(), MAX_EVENTS as i32, -1);
            if num_events < 0 {
                eprintln!("epoll_wait failed: {:?}", io::Error::last_os_error());
                continue;
            }

            for i in 0..num_events as usize {
                let event = &events[i];
                if (event.events & EPOLLIN as u32) != 0 {
                    // Accepte la connexion
                    let client_fd = accept(sockfd, ptr::null_mut(), ptr::null_mut());
                    if client_fd < 0 {
                        eprintln!("Failed to accept connection: {:?}", io::Error::last_os_error());
                        continue;
                    }

                    // Lit la requête HTTP
                    let mut buffer = [0; 512];
                    let bytes_read = libc::read(client_fd, buffer.as_mut_ptr() as *mut _, buffer.len() as libc::size_t);
                    if bytes_read <= 0 {
                        eprintln!("Failed to read from client: {:?}", io::Error::last_os_error());
                        libc::close(client_fd);
                        continue;
                    }

                    let request = &buffer[..bytes_read as usize];
                    let (method, path) = parse_http_request(request);

                    // Gère la requête
                    let response = if path.starts_with(CGI_PATH) {
                        handle_cgi_request(&path)
                    } else {
                        match method.as_str() {
                            "GET" => handle_get_request(&path),
                            "POST" => handle_post_request(request),
                            "DELETE" => handle_delete_request(&path),
                            _ => format_http_response("405", "Method Not Allowed", "Method not allowed"),
                        }
                    };

                    // Envoie la réponse HTTP
                    libc::write(client_fd, response.as_bytes().as_ptr() as *const _, response.len() as libc::size_t);
                    libc::close(client_fd);
                }
            }
        }
    }
}

// Fonction pour analyser les requêtes HTTP
fn parse_http_request(request: &[u8]) -> (String, String) {
    let request_str = std::str::from_utf8(request).unwrap_or("");
    let mut lines = request_str.lines();
    let first_line = lines.next().unwrap_or("");
    let mut parts = first_line.split_whitespace();
    let method = parts.next().unwrap_or("").to_string();
    let path = parts.next().unwrap_or("").to_string();
    (method, path)
}

// Fonction pour formater les réponses HTTP
fn format_http_response(status_code: &str, status_text: &str, body: &str) -> String {
    format!(
        "HTTP/1.1 {} {}\r\nContent-Length: {}\r\nContent-Type: text/html\r\n\r\n{}",
        status_code,
        status_text,
        body.len(),
        body
    )
}

// Fonction pour gérer les requêtes GET
fn handle_get_request(path: &str) -> String {
    // Supprime le slash initial du chemin pour faciliter les recherches
    let path_trimmed = path.trim_start_matches('/');

    // Liste des variantes de chemins à essayer
    let possible_paths = vec![
        path_trimmed.to_string(),
        format!("{}.html", path_trimmed),
        format!("{}.htm", path_trimmed),
        format!("{}.py", path_trimmed),
        format!("{}.php", path_trimmed),
    ];

    for path in possible_paths {
        let body = match read_file(&path) {
            Ok(content) => content,
            Err(_) => continue,
        };

        return format_http_response("200", "OK", &body);
    }

    // Si aucun fichier n'a été trouvé, renvoie 404 Not Found
    format_http_response("404", "Not Found", "404 Not Found")
}

// Fonction pour lire les fichiers
fn read_file(file_path: &str) -> Result<String, io::Error> {
    let path = Path::new("public").join(file_path.trim_start_matches('/'));
    let mut file = File::open(&path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// Fonction pour gérer les requêtes POST
fn handle_post_request(_request: &[u8]) -> String {
    let body = "POST request received".to_string();
    format_http_response("200", "OK", &body)
}

// Fonction pour gérer les requêtes DELETE
fn handle_delete_request(path: &str) -> String {
    let body = format!("Deleted: {}", path);
    format_http_response("200", "OK", &body)
}

// Fonction pour gérer les requêtes CGI
fn handle_cgi_request(path: &str) -> String {
    // Enlève le préfixe CGI_PATH pour obtenir le chemin du script
    let script_path = path.trim_start_matches(CGI_PATH);

    // Exécute le script CGI
    let output = Command::new("python3")
        .arg(script_path)
        .output()
        .expect("Failed to execute CGI script");

    // Gère la sortie du script CGI
    if !output.status.success() {
        return format_http_response("500", "Internal Server Error", "CGI script failed");
    }

    // Formate la réponse avec la sortie du script CGI
    format_http_response("200", "OK", &String::from_utf8_lossy(&output.stdout))
}
