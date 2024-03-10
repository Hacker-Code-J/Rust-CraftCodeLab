// storage.rs

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File, io::{self, Read, Write}, path::Path};
use crate::password::Password; // Assuming the Password struct provides necessary encryption/decryption

#[derive(Serialize, Deserialize)]
pub struct PasswordStore {
    passwords: HashMap<String, String>, // Key-value pair of identifier and encrypted password
}

impl PasswordStore {
    // Create a new, empty PasswordStore
    pub fn new() -> Self {
        PasswordStore {
            passwords: HashMap::new(),
        }
    }

    // Load the password store from a file
    pub fn load<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let store: PasswordStore = serde_json::from_str(&contents)?;
        Ok(store)
    }

    // Save the password store to a file
    pub fn save<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let contents = serde_json::to_string(&self)?;
        let mut file = File::create(path)?;
        file.write_all(contents.as_bytes())
    }

    // Add a new password to the store, encrypting it before storage
    pub fn add(&mut self, identifier: String, password: &Password) -> Result<(), &'static str> {
        if self.passwords.contains_key(&identifier) {
            return Err("Identifier already exists");
        }
        // Here you should implement encryption with the real password value before storing
        let encrypted = password.encrypt();
        self.passwords.insert(identifier, encrypted);
        Ok(())
    }

    // Retrieve an encrypted password from the store and decrypt it
    pub fn get(&self, identifier: &str) -> Result<Password, &'static str> {
        if let Some(encrypted) = self.passwords.get(identifier) {
            // Here you should implement decryption and then return
            Ok(Password::decrypt(encrypted))
        } else {
            Err("Identifier not found")
        }
    }
}