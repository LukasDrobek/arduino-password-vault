use std::collections::HashMap;

#[derive(Debug)]
pub struct PasswordEntry {
    service: String,
    username: String,
    password: String,
}

#[derive(Debug)]
pub struct Vault {
    entries: HashMap<(String, String), PasswordEntry>
}

impl Vault {
    pub fn new() -> Self {
        Vault { entries: HashMap::new() }
    }

    pub fn add(&mut self, service: String, username: String, password: String) -> bool {
        let key = (service.clone(), username.clone());
        if self.entries.contains_key(&key) {
            return false;
        } else {
            self.entries.insert(key, PasswordEntry { service, username, password });
            return true;
        }
    }

    pub fn get_all(&self) -> Vec<&PasswordEntry> {
        self.entries.values().collect()
    }

    pub fn find(&self, service: String, username: Option<String>) -> Vec<&PasswordEntry> {
        match username {
            Some(name) => {
                self.entries
                    .get(&(service, name))
                    .into_iter()
                    .collect()
            }
            None => {
                self.entries
                    .iter()
                    .filter_map(|((s, _), entry)| if *s == service { Some(entry) } else { None })
                    .collect()
            }
        }   
    }
}