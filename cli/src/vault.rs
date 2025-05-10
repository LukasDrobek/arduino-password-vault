use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Debug, Clone, Serialize, Deserialize, Zeroize, ZeroizeOnDrop)]
pub struct PasswordEntry {
    service: String,
    username: String,
    password: String,
}

impl PasswordEntry {
    pub fn service(&self) -> &str {
        &self.service
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn password(&self) -> &str {
        &self.password
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordVault {
    version: u8,
    entries: HashMap<String, PasswordEntry>,
}

impl PasswordVault {
    pub fn new() -> Self {
        PasswordVault {
            version: 1,
            entries: HashMap::new(),
        }
    }

    pub fn add(&mut self, service: &str, username: &str, password: &str) -> Option<PasswordEntry> {
        let key = format!("{}|{}", service, username);
        if self.entries.contains_key(&key) {
            return None;
        }
        self.entries.insert(
            key,
            PasswordEntry {
                service: service.to_string(),
                username: username.to_string(),
                password: password.to_string(),
            },
        )
    }

    pub fn get(
        &mut self,
        service: Option<String>,
        username: Option<String>,
    ) -> Vec<&PasswordEntry> {
        match (service, username) {
            (None, _) => self.entries.values().collect(),
            (Some(service), None) => self
                .entries
                .iter()
                .filter_map(|(s, entry)| if *s == service { Some(entry) } else { None })
                .collect(),
            (Some(service), Some(username)) => {
                let key = format!("{}|{}", service, username);
                self.entries.get(&key)
                    .into_iter()
                    .collect()
            },
        }
    }

    pub fn delete(&mut self, service: &str, username: &str) -> Option<PasswordEntry> {
        let key = format!("{}|{}", service, username);
        if !self.entries.contains_key(&key) {
            return None;
        }
        self.entries.remove(&key)
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
