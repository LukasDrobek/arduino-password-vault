#[derive(Debug)]
pub struct PasswordEntry {
    service: String,
    username: String,
    password: String,
}

#[derive(Debug)]
pub struct Vault {
    entries: Vec<PasswordEntry>
}

impl Vault {
    pub fn new() -> Self {
        Vault { entries: Vec::new() }
    }

    pub fn add_entry(&mut self, service: String, username: String, password: String) {
        self.entries.push(PasswordEntry { service, username, password });
    }

    pub fn list_all_entries(&self) {
        for (i, entry) in self.entries.iter().enumerate() {
            println!("{}. {} - {}", i + 1, entry.service, entry.username);
        }
    }

    pub fn find_entries(&self, service: String) {
        let filtered_entries : Vec<&PasswordEntry> = self.entries
            .iter()
            .filter(|entry| entry.service == service)
            .collect();
        for (i, entry) in filtered_entries.iter().enumerate() {
            println!("{}. {} - {}", i + 1, entry.service, entry.username);
        }
    }

    pub fn get_password(&self, service: String, username: String) {
        let filtered_entries : Vec<&PasswordEntry> = self.entries
            .iter()
            .filter(|entry| entry.service == service && entry.username == username)
            .collect();
        if filtered_entries.is_empty() {
            println!("No password found for {} - {}", service, username);
        }
        for (_, entry) in filtered_entries.iter().enumerate() {
            println!("{} - {}", entry.service, entry.username);
            println!("-> {}", entry.password);
        }
    }

}