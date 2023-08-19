use aead::{Aead, AeadCore, KeyInit};
use aes_gcm::{Aes256Gcm, Nonce, Key};
use generic_array::GenericArray;
use argon2::{
    password_hash::{
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};
use rand_core::OsRng;
use serde::{Serialize, Deserialize};
use sha3::Sha3_512;


const NONCE_SIZE: usize = 12;
const KEY_SIZE: usize = 32;

fn derive_key_from_master_password(master_password: &str, salt: &[u8; NONCE_SIZE]) -> Result<[u8; KEY_SIZE], &'static str> {
    let mut key: [u8; KEY_SIZE] = [0u8; KEY_SIZE];

    Argon2::default().hash_password_into(master_password.as_bytes(), salt, &mut key).map(|_| key).map_err(|_| "Key derive failed")
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

    pub fn update_username(&mut self, new_username: String) {
        self.username = new_username;
    }

    pub fn update_fields(&mut self, new_username: Option<String>, new_password: Option<String>, master_password: &str) {
        if let Some(username) = new_username {
            self.username = username;
        }

        if let Some(password) = new_password {
            if let Err(err) = self.encrypt_password(&password, master_password) {
                println!("Failed to encrypt new password: {}", err);
            }
        }
    }

    pub fn encrypt_password(&mut self, plaintext_password: &str, master_password: &str) -> Result<(), String> {
        let key: [u8; 32] = derive_key_from_master_password(master_password, &self.nonce).map_err(|_| "Failed to derive key from master password")?;
        let cipher = Aes256Gcm::new(&key.into());

        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

        let ciphertext: Vec<u8> = cipher.encrypt(
            &nonce,
            plaintext_password.as_ref(),
        ).map_err(|_| "Encryption failed")?;

        self.encrypted_password = ciphertext;
        self.nonce = nonce.into();

        Ok(())
    }

    pub fn decrypt_password(&self, master_password: &str) -> Result<String, String> {
        let key: [u8; 32] = derive_key_from_master_password(master_password, &self.nonce).map_err(|_| "Failed to derive key from master password")?;
        let cipher = Aes256Gcm::new(&key.into());

        let plaintext = cipher.decrypt(
            GenericArray::from_slice(&self.nonce),
            self.encrypted_password.as_ref(),
        ).map_err(|_| "Decryption failed")?;

        Ok(String::from_utf8_lossy(&plaintext).to_string())
    }
}


