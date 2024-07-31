# Localhosh

Localhosh est un serveur HTTP simple en Rust qui prend en charge les requêtes HTTP/1.1 et les scripts CGI. Il peut gérer des requêtes GET, POST et DELETE, ainsi que des fichiers statiques et des scripts CGI.

## Fonctionnalités

- **Serveur HTTP/1.1** : Supporte les méthodes GET, POST et DELETE.
- **Fichiers statiques** : Sert des fichiers HTML, HTM, PHP et PY à partir d'un répertoire public.
- **CGI** : Exécute des scripts CGI en fonction de leur extension.
- **Port configurable** : Spécifiez le port sur lequel le serveur écoute.

## Prérequis

- **Rust** : Assurez-vous d'avoir Rust installé. Vous pouvez l'installer depuis [rust-lang.org](https://www.rust-lang.org/).
- **Python 3** : Pour exécuter les scripts CGI en Python. Assurez-vous que Python 3 est installé.

## Installation

Clonez le dépôt et construisez le projet avec Cargo :

```bash
git clone <URL_DU_REPOSITORY>
cd localhosh
cargo build
```

## Utilisation

### Lancer le serveur

Pour démarrer le serveur, utilisez la commande suivante :

```bash
cargo run -- --port <PORT>
```

Remplacez  par le numéro de port sur lequel vous souhaitez que le serveur écoute. Par défaut, le serveur écoute sur le port 8080.

### Exemple

Pour démarrer le serveur sur le port 8081 :

```bash
cargo run -- --port 8081
```

### Structure du Répertoire

- **public/** : Répertoire pour les fichiers statiques (HTML, HTM, PHP, PY).
- **CGI** : Les scripts CGI doivent être placés dans un répertoire spécifique (par exemple, `cgi-bin`).

### Formats des Requêtes

- **GET** : Accède aux fichiers statiques ou exécute des scripts CGI.
- **POST** : Accepte les requêtes POST et renvoie une réponse fixe.
- **DELETE** : Supprime les ressources et renvoie une confirmation.

### Exécution des Scripts CGI

Les scripts CGI doivent être placés dans le répertoire configuré pour les CGI (par exemple, `/cgi-bin/`). Le serveur détecte les scripts en fonction de leur extension (comme `.py` pour Python). Les scripts doivent être exécutables et accessibles.

## Exemple de Commande CGI

Pour tester un script CGI en Python :

1. Placez votre script Python dans le répertoire `public/cgi-bin/` et assurez-vous qu'il est exécutable.

2. Accédez à votre script via une requête GET, par exemple :

   ```bash
   curl http://127.0.0.1:8080/cgi-bin/mon_script.py
   ```

## Dépannage

- **Erreur "CGI script not found"** : Assurez-vous que le script CGI est dans le bon répertoire et qu'il est exécutable.
- **Erreur "No such file or directory"** : Vérifiez les chemins des fichiers et les permissions des répertoires.

## Authors
(ediallo)[https://learn.zone01dakar.sn/git/ediallo] 
