use std::env;
use std::process::Command;
use serde_json::Value;
use hex;

fn main() {
    // Récupère les arguments de la ligne de commande
    let args: Vec<String> = env::args().collect();

    // Si un seul argument est fourni, on traite un seul bloc
    let (start_block, end_block) = match args.len() {
        2 => {
            let block: u64 = args[1].parse().expect("Le bloc doit être un nombre.");
            (block, block) // Le bloc de début et de fin sont le même
        },
        3 => {
            let start_block: u64 = args[1].parse().expect("Le bloc de début doit être un nombre.");
            let end_block: u64 = args[2].parse().expect("Le bloc de fin doit être un nombre.");
            if start_block > end_block {
                eprintln!("Le bloc de début doit être inférieur ou égal au bloc de fin.");
                return;
            }
            (start_block, end_block)
        },
        _ => {
            eprintln!("Usage: cargo run <start_block> [<end_block>]");
            return;
        }
    };

    // Boucle sur la plage de blocs entre start_block et end_block
    for block_number in start_block..=end_block {
        println!("Vérification du bloc #{}", block_number);
        
        match process_block(block_number) {
            Ok(Some(coinbase_text)) => {
                println!("Coinbase (texte lisible du bloc #{}): {}", block_number, coinbase_text);
            }
            Ok(None) => {
                println!("Aucune transaction coinbase trouvée dans le bloc #{}", block_number);
            }
            Err(err) => {
                eprintln!("Erreur dans le bloc #{}: {}", block_number, err);
            }
        }
    }
}

// Fonction pour traiter un bloc et afficher son contenu coinbase
fn process_block(block_number: u64) -> Result<Option<String>, String> {
    // Récupérer le hash du bloc correspondant
    let block_hash = get_block_hash(block_number).unwrap_or_else(|err| {
        eprintln!("Erreur en récupérant le hash du bloc: {}", err);
        std::process::exit(1);
    });
    
    // Récupérer les données du bloc
    let block_data = get_block_data(&block_hash).unwrap_or_else(|err| {
        eprintln!("Erreur en récupérant les données du bloc: {}", err);
        std::process::exit(1);
    });
    
    // Vérifier et décoder le champ coinbase
    if let Some(coinbase_hex) = get_coinbase_data(&block_data) {
        match decode_coinbase(&coinbase_hex) {
            Ok(decoded) => Ok(Some(decoded)),
            Err(err) => Err(format!("Erreur lors du décodage du coinbase: {}", err)),
        }
    } else {
        Ok(None)
    }
}

// Fonction pour récupérer le hash d'un bloc à partir de son numéro
fn get_block_hash(block_number: u64) -> Result<String, String> {
    let output = Command::new("bitcoin-cli")
        .arg("getblockhash")
        .arg(block_number.to_string())
        .output()
        .expect("Échec lors de l'exécution de bitcoin-cli");

    if !output.status.success() {
        return Err(format!(
            "Erreur dans getblockhash: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

// Fonction pour récupérer les données d'un bloc à partir de son hash
fn get_block_data(block_hash: &str) -> Result<Value, String> {
    let output = Command::new("bitcoin-cli")
        .arg("getblock")
        .arg(block_hash)
        .arg("2") // Demander les données complètes du bloc
        .output()
        .expect("Échec lors de l'exécution de bitcoin-cli");

    if !output.status.success() {
        return Err(format!(
            "Erreur dans getblock: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let block_data: Value = serde_json::from_slice(&output.stdout)
        .map_err(|e| format!("Erreur lors de l'analyse des données du bloc: {}", e))?;
    Ok(block_data)
}

// Fonction pour extraire le champ coinbase de la première transaction (coinbase transaction)
fn get_coinbase_data(block_data: &Value) -> Option<String> {
    let tx = block_data["tx"].get(0)?;
    let vin = tx["vin"].get(0)?;
    vin["coinbase"].as_str().map(|s| s.to_string())
}

// Fonction pour décoder la chaîne coinbase hexadécimale en texte lisible
fn decode_coinbase(coinbase_hex: &str) -> Result<String, String> {
    let decoded_bytes = hex::decode(coinbase_hex).map_err(|e| format!("Erreur lors du décodage hex: {}", e))?;
    let decoded_string = String::from_utf8_lossy(&decoded_bytes);
    Ok(decoded_string.to_string())
}
