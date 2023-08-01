use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Nonce, Key
};
use pbkdf2::pbkdf2_hmac;
use rand::{Rng, thread_rng};
use serde::{Serialize, Deserialize};
use sha2::Sha256;


const NONCE_SIZE: usize = 12;
const KEY_SIZE: usize = 32;

fn derive_key_from_master_password(master_password: &str, salt: &[u8;16]) -> [u8; KEY_SIZE] {
    let mut key = [0u8; KEY_SIZE];
    let rounds: u32 = 100_000;

    pbkdf2_hmac::<Sha256>(master_password.as_bytes(), salt, rounds, &mut key);
    
    key
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordEntry {
    website: String,
    username: String,
    encrypted_password: Vec<u8>,
    nonce: [u8; NONCE_SIZE],
}


impl PasswordEntry {
    pub fn new(website: String, username: String, encrypted_password: Vec<u8>) -> Self{
        PasswordEntry {
            website, 
            username,
            encrypted_password,
            nonce: [0u8; NONCE_SIZE],
        }
    }

    pub fn website(&self) -> &str {
        &self.website
    }
    
    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn encrypt_password(&mut self, plaintext_password: &str, key: &[u8; KEY_SIZE]) -> Result<(), String> {
        let key = Key::<Aes256Gcm>::from_slice(key);
        let cipher = Aes256Gcm::new(&key);

        let mut nonce = [0u8; NONCE_SIZE];
        thread_rng().fill(&mut nonce);

        let ciphertext = cipher.encrypt(
            GenericArray::from_slice(&nonce),
            Payload {
                msg: plaintext_password.as_bytes(),
                aad: &[],
            },
        ).map_err(|_| "Encryption failed")?;

        self.encrypted_password = ciphertext;
        self.nonce = nonce;

        Ok(())
    }

    pub fn decrypt_password(&self, key: &[u8; KEY_SIZE]) -> Result<String, String> {
        let cipher = Aes256Gcm::new(GenericArray::from_slice(key));

        let plaintext = cipher.decrypt(
            GenericArray::from_slice(&self.nonce),
            Payload {
                msg: &self.encrypted_password,
                aad: &[],
            },
        ).map_err(|_| "Decryption failed")?;

        Ok(String::from_utf8_lossy(&plaintext).to_string())
    }
}


