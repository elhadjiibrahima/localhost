# Diagramme de sequence sur localhost
    - 1. Client (Navigateur) envoie une requête HTTP à l'adresse IP et au port spécifiés.
    - 2. Serveur HTTP (Rust) reçoit la requête via epoll (ou équivalent).
    - 3. Le Serveur HTTP traite la requête en fonction du type (GET, POST, DELETE).
    - 4. Si la requête est pour un fichier statique :
        - Le Serveur HTTP vérifie la présence du fichier dans le répertoire spécifié.
        - Si le fichier est trouvé, le Serveur HTTP envoie une réponse HTTP avec le corps du fichier.
        - Si le fichier n'est pas trouvé, le Serveur HTTP envoie une page d'erreur par défaut (par exemple, 404).
    - 5. Si la requête est pour un fichier dynamique (CGI) :
        - Le Serveur HTTP exécute le fichier CGI correspondant à l'extension du fichier demandé.
        - Le Serveur HTTP envoie la sortie du CGI en tant que réponse HTTP.
    - 6. Le Serveur HTTP gère les cookies et les sessions si nécessaire.
    - 7. Le Serveur HTTP envoie la réponse HTTP au Client (Navigateur).
    - 8. Le Client (Navigateur) affiche la réponse (page web, message, etc.).

Le diagramme de séquence détaillé pour chaque étape peut être réalisé à l'aide d'un outil comme PlantUML ou tout autre logiciel de création de diagrammes UML.