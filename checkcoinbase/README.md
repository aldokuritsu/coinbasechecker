# Bitcoin Coinbase Checker

## Description

Ce programme en Rust permet de vérifier le contenu du champ `coinbase` des transactions des blocs Bitcoin. Le champ `coinbase` contient généralement des informations spécifiques aux mineurs, comme des messages ou des identifiants techniques. Ce script utilise le client `bitcoin-cli` pour interroger un nœud complet Bitcoin et extraire les données des blocs demandés.

Vous pouvez utiliser ce script pour :
- Vérifier le contenu du champ `coinbase` d'un bloc spécifique.
- Parcourir une plage de blocs et afficher les champs `coinbase`.

## Prérequis

- Un nœud Bitcoin complet configuré pour accepter les requêtes RPC (via `bitcoin-cli`).
- Rust installé sur votre système.
- `bitcoin-cli` installé et accessible via la ligne de commande.

## Installation

1. Clonez le dépôt ou copiez les fichiers du script dans un projet Rust.

2. Ajoutez les dépendances suivantes dans votre fichier `Cargo.toml` :

   ```toml
   [dependencies]
   serde = { version = "1.0", features = ["derive"] }
   serde_json = "1.0"
   hex = "0.4"
   ```

## Utilisation

Le programme prend un ou deux arguments :

- Si un seul argument est fourni, il vérifie le contenu du champ coinbase du bloc spécifié.
- Si deux arguments sont fournis, il vérifie le contenu des blocs compris entre les deux valeurs (inclusivement).

### Exemple de commande :

- Pour vérifier le champ coinbase du bloc 100 :

    ```
    cargo run 100
    ``` 

- Pour vérifier le champ coinbase du bloc 100 à 200 :

    ```
    cargo run 100 200
    ``` 
Le programme affichera les contenus des champs coinbase en texte lisible pour chaque bloc demandé.

## Erreurs possibles

- Si le script ne peut pas récupérer les données d'un bloc, il affichera un message d'erreur expliquant la raison de l'échec (par exemple, si le bloc est invalide ou si le nœud Bitcoin est inaccessible).

### Exemple de sortie :

    ```
    Vérification du bloc #0
    Coinbase (texte lisible du bloc #100): The Times 03/Jan/2009 Chancellor on brink of second bailout for banks.
    ``` 

## Notes

- Le champ coinbase peut contenir des données en hexadécimal qui doivent être décodées en texte lisible, ce que le programme fait automatiquement.

- Le programme utilise bitcoin-cli, qui doit être configuré correctement pour fonctionner avec votre nœud Bitcoin.