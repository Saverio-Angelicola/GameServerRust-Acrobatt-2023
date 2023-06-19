# Serveur du jeu VegaFLag

## Contexte :
Le projet est un serveur de jeu multijoueur dans le cadre du projet Acrobatt de la licence pro CDAD lors de l'année 2023.

Le jeu consiste à dominer des territoires en équipes en capturant des drapeaux. Des items permettent de donner des avantages au équipes les ramassant afin de rendre la partie plus dynamique.

<br>

## Prérequis
- Docker
- Rustup
- Editeur de code

<br>

## Build

Cargo : `cargo build --release`

Docker : `docker build -t gameserver .`

<br>

## Run

Cargo : `cargo run -- port`

docker : `docker run -p port:port -p 5050:5050 -d gameserver`

<br>

## Structure

Dans le dossier Src :

- Core : Contient les éléments du jeu comme les structures de données du jeu et la logique de jeu.
<br>

- Server : Contient l'implémentation des commandes ainsi que la configuration du serveur et le chargement des fichiers de configuration du jeu et de la carte.
<br>

 - shared : Contient des structures de données partagés plus générique.

 ## Démarrage

 Pour démarrer le développement de nouvelles fonctionnalités ou modifier les fonctionnalités existantes, vous devez commencer par vous rendre dans le ficheir src/server/server_command.rs. Dans ce fichier vous trouverez l'ensemble des commandes contenant les fonctionnalités de l'application.

 Pour ajouter une nouvelle fonctionnalité, il suffit d'ajouter une nouvelle commande et d'écrire la logique associé à la commande.