use std::time::SystemTime;

use aead::{Aead, AeadCore, KeyInit};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use generic_array::GenericArray;
use rand_core::OsRng;
use serde::{Deserialize, Serialize};
use sha3::Sha3_512;
use std::cmp::Ordering;

const NONCE_SIZE: usize = 12;
const KEY_SIZE: usize = 32;

pub fn derive_key_from_master_password(
    master_password: &str,
    salt: &[u8; NONCE_SIZE],
) -> Result<[u8; KEY_SIZE], &'static str> {
    let mut key: [u8; KEY_SIZE] = [0u8; KEY_SIZE];

    Argon2::default()
        .hash_password_into(master_password.as_bytes(), salt, &mut key)
        .map(|_| key)
        .map_err(|_| "Key derive failed")
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct PasswordEntry {
    website: String,
    username: String,
    encrypted_password: Vec<u8>,
    nonce: [u8; NONCE_SIZE],
    timestamp: SystemTime,
}

impl PasswordEntry {
    pub fn new(website: String, username: String, encrypted_password: Vec<u8>) -> Self {
        PasswordEntry {
            website,
            username,
            encrypted_password,
            nonce: [0u8; NONCE_SIZE],
            timestamp: SystemTime::now(),
        }
    }

    pub fn website(&self) -> &str {
        &self.website
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn encrypt_password(
        &mut self,
        plaintext_password: &str,
        master_password: &str,
    ) -> Result<(), String> {
        let key: [u8; 32] = derive_key_from_master_password(master_password, &self.nonce)
            .map_err(|_| "Failed to derive key from master password")?;
        let cipher = Aes256Gcm::new(&key.into());

        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

        let ciphertext: Vec<u8> = cipher
            .encrypt(&nonce, plaintext_password.as_ref())
            .map_err(|_| "Encryption failed")?;

        self.encrypted_password = ciphertext;
        self.nonce = nonce.into();

        Ok(())
    }

    pub fn decrypt_password(&self, master_password: &str) -> Result<String, String> {
        let key: [u8; 32] = derive_key_from_master_password(master_password, &self.nonce)
            .map_err(|_| "Failed to derive key from master password")?;
        let cipher = Aes256Gcm::new(&key.into());

        let plaintext = cipher
            .decrypt(
                GenericArray::from_slice(&self.nonce),
                self.encrypted_password.as_ref(),
            )
            .map_err(|_| "Decryption failed")?;

        Ok(String::from_utf8_lossy(&plaintext).to_string())
    }
}

impl Ord for PasswordEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        let website_ordering = self.website.cmp(&other.website);
        if website_ordering != Ordering::Equal {
            return website_ordering;
        }
        // If websites are equal, compare based on username
        let username_ordering = self.username.cmp(&other.username);
        if username_ordering != Ordering::Equal {
            return username_ordering;
        }

        // If both website and username are equal, compare based on timestamp
        other.timestamp.cmp(&self.timestamp)
    }
}

impl PartialOrd for PasswordEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
