use aljam3_connections::db::Aljam3Db;
use aljam3_connections::messaging::EncryptedNatsClient;
use aljam3_connections::conversions::{to_json, to_toon};
use aljam3_connections::crypto::{encrypt_payload, decrypt_payload};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct DemoPayload {
    message: String,
    timestamp: i64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- Aljam3 Connections Demo ---\n");

    // 1. Serialization Demo (JSON -> TOON)
    let payload = DemoPayload {
        message: "Hello Aljam3!".to_string(),
        timestamp: 1672531200,
    };
    
    let json_str = to_json(&payload)?;
    println!("[Serializers] JSON representation: {}", json_str);
    
    let toon_str = to_toon(&payload)?;
    println!("[Serializers] TOON representation: {}", toon_str);

    // 2. Encryption Demo
    let dummy_key = [0u8; 32]; // 32-byte key for AES-256
    let encrypted = encrypt_payload(json_str.as_bytes(), &dummy_key)?;
    println!("[Encryption] Encrypted payload length (with nonce): {} bytes", encrypted.len());
    
    let decrypted = decrypt_payload(&encrypted, &dummy_key)?;
    println!("[Encryption] Decrypted payload: {}", String::from_utf8(decrypted)?);

    // 3. Database Check (Assuming PostgreSQL is running on local dev setup)
    println!("\n[PostgreSQL] Attempting to connect to NoSQL JSONB engine...");
    let db_url = "postgres://aljam3:aljam3@localhost:5432/aljam3";
    
    match Aljam3Db::connect(db_url).await {
        Ok(db) => {
            if db.check_existence().await? {
                println!("  ✅ PostgreSQL is online!");
                
                // Initialize DB
                println!("  ⚙️  Initializing Mega Metadata Tree schema...");
                db.initialize_db().await?;
                
                if db.check_metadata_tree_topology().await? {
                    println!("  ✅ Metadata Tree topology exists.");
                }
            }
        },
        Err(e) => {
            println!("  ❌ Could not connect to PostgreSQL. Is the dev infrastructure running? ({})", e);
        }
    }

    // 4. NATS Messaging Check
    println!("\n[NATS] Attempting to connect to async message broker...");
    match EncryptedNatsClient::connect("nats://localhost:4222", &dummy_key).await {
        Ok(client) => {
            println!("  ✅ NATS connected!");
            
            println!("  📤 Publishing encrypted test message to 'aljam3.trigger'...");
            client.publish("aljam3.trigger", b"Test Trigger Data").await?;
            println!("  ✅ Published successfully.");
        },
        Err(e) => {
            println!("  ❌ Could not connect to NATS. Is the dev infrastructure running? ({})", e);
        }
    }

    Ok(())
}
