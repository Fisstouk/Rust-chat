# Rust-chat

Logiciel de chat en temps réel, à utiliser en ligne de commande

## Ce qui fonctionne

Dans la brance `main` vous trouverez les éléments suivants:
* Une arborescence avec un serveur et un client dans des dossiers distincts
* Un serveur fonctionnel avec la possibilité de lancer en même temps deux clients
* Il est possible de lancer le serveur et les deux clients sur la même machine, dans des terminaux différents
* Le programme demande à l'utilisateur son pseudo et l'affiche à chaque nouveau message

## Ce qui doit être amélioré

### Branche `Mathis`

* Chiffrement à la main qui résulte par un panic dès les messages sont échangés

### Branche `lyronn-encrypt`

* Utilisation de la bibliothèque `openssl::rsa`
* Erreur au niveau des signatures des fonctions `encrypt` et `decrypt`: cannot find type `T` in this scope
not found in this scope`

### Branche `feature/username`

* Proposer à l'utilisateur un choix de couleur qui sera affichée pour son pseudo
