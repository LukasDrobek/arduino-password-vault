use std::collections::HashMap;
use zeroize::{Zeroize, ZeroizeOnDrop};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Zeroize, ZeroizeOnDrop)]
pub struct PasswordEntry {
    service: String,
    username: String,
    password: String,
}

impl PasswordEntry {
    pub fn new(service: &str, username: &str, password: &str) -> Self {
        PasswordEntry { 
            service: service.to_string(), 
            username: username.to_string(),
            password: password.to_string()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordVault {
    version: u8,
    entries: HashMap<(String, String), PasswordEntry>
}

impl PasswordVault {
    pub fn new() -> Self {
        PasswordVault { version: 1, entries: HashMap::new() }
    }

    pub fn add(&mut self, entry: PasswordEntry) -> Option<PasswordEntry> {
        self.entries.insert((entry.service.clone(), entry.username.clone()), entry)
    }

    pub fn get(&mut self, service: Option<String>, username: Option<String>) -> Vec<&PasswordEntry> {
        match (service, username) {
            (None, _) => {
                self.entries.values().collect()
            }
            (Some(service), None) => {
                self.entries
                    .iter()
                    .filter_map(|((s, _), entry)| if *s == service { Some(entry) } else { None })
                    .collect()
            }
            (Some(service), Some(username)) => {
                self.entries
                    .get(&(service, username))
                    .into_iter()
                    .collect()
            }
        }
    }

    pub fn delete(&mut self, service: &str, username: &str) -> Option<PasswordEntry> {
        self.entries.remove(&(service.to_string(), username.to_string()))
    }
}

impl Zeroize for PasswordVault {
    fn zeroize(&mut self) {
        for entry in self.entries.values_mut() {
            entry.zeroize();
        }
        self.entries.clear();
    }
}

impl Drop for PasswordVault {
    fn drop(&mut self) {
        self.zeroize();
    }
}