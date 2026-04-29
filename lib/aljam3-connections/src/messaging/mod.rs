use async_nats::{Client, Subscriber};
use std::error::Error;
use bytes::Bytes;
use crate::crypto::{encrypt_payload, decrypt_payload};

/// A wrapper around the async-nats client that natively supports
/// payload encryption for all outgoing and incoming messages.
pub struct EncryptedNatsClient {
    client: Client,
    encryption_key: Vec<u8>,
}

impl EncryptedNatsClient {
    /// Connects to a NATS server and initializes the client wrapper with a symmetric encryption key.
    ///
    /// # Arguments
    /// * `url` - The NATS server URL (e.g. "nats://localhost:4222").
    /// * `encryption_key` - The 32-byte symmetric key for payload encryption.
    pub async fn connect(url: &str, encryption_key: &[u8]) -> Result<Self, Box<dyn Error>> {
        if encryption_key.len() != 32 {
            return Err("Encryption key must be exactly 32 bytes".into());
        }
        
        let client = async_nats::connect(url).await?;
        
        Ok(Self {
            client,
            encryption_key: encryption_key.to_vec(),
        })
    }
    
    /// Encrypts the payload and publishes it to the specified subject.
    pub async fn publish(&self, subject: &str, payload: &[u8]) -> Result<(), Box<dyn Error>> {
        // Encrypt the payload before sending
        let encrypted_payload = encrypt_payload(payload, &self.encryption_key)?;
        
        // Publish via NATS
        self.client.publish(subject.to_string(), Bytes::from(encrypted_payload)).await?;
        
        Ok(())
    }
    
    /// Subscribes to a subject and returns a raw Subscriber stream.
    ///
    /// Note: Callers will need to decrypt the incoming messages manually using `decrypt_message`,
    /// or we can provide a wrapper stream if needed.
    pub async fn subscribe(&self, subject: &str) -> Result<Subscriber, Box<dyn Error>> {
        let subscriber = self.client.subscribe(subject.to_string()).await?;
        Ok(subscriber)
    }
    
    /// Helper utility to decrypt a received NATS message payload.
    pub fn decrypt_message(&self, encrypted_payload: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
        decrypt_payload(encrypted_payload, &self.encryption_key)
    }
}
