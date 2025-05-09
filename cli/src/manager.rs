use anyhow::{anyhow, Result};
use zeroize::Zeroizing;
use rand::RngCore;
use rand::rngs::OsRng;

use crate::constants::{MASTER_KEY_LEN, SALT_LEN};
use crate::crypto;
use crate::vault::{PasswordEntry, PasswordVault};
use crate::serial::SerialManager;

pub struct VaultManager {
    serial: SerialManager,
    master_key: Option<Zeroizing<[u8; MASTER_KEY_LEN]>>,
    vault: Option<Zeroizing<PasswordVault>>,
    is_init: bool,
    is_locked: bool,
    needs_update: bool
}

impl VaultManager {
    pub fn new() -> Result<Self> {
        let serial = SerialManager::new()?;
        
        Ok(Self {
            serial,
            master_key: None,
            vault: None,
            is_init: false,
            is_locked: true,
            needs_update: false
        })
    }

    pub fn init(&mut self, password: &str) -> Result<()> {
        // generate salt and derive master key
        let mut salt = [0u8; SALT_LEN];
        OsRng.fill_bytes(&mut salt);
        let master_key = crypto::dervive_key(password, &salt)?;

        // initialize empty vault
        let password_vault = PasswordVault::new();
        let password_vault_json = serde_json::to_string(&password_vault)?;

        // encrypt vault
        let (nonce, ciphertext, auth_tag) = crypto::encrypt_data(&master_key, password_vault_json.as_bytes())?;

        // send salt to arduino (hex)
        let init_salt_command = format!(
            "INIT_SALT:{}\n",
            hex::encode(salt)
        );
        self.serial.write(&init_salt_command)?;

        // send encrypted vault to arduino (hex)
        let init_vault_command = format!(
            "INIT_VAULT:{}:{}:{}\n",
            hex::encode(nonce),
            hex::encode(ciphertext),
            hex::encode(auth_tag)
        );
        self.serial.write(&init_vault_command)?;

        // update state
        self.master_key = Some(Zeroizing::new(master_key));
        self.vault = Some(Zeroizing::new(password_vault));
        self.is_init = true;
        self.is_locked = false;
        self.needs_update = false;

        Ok(())
    }

    pub fn is_init(&self) -> bool {
        self.is_init
    }

    pub fn is_locked(&self) -> bool {
        self.is_locked
    }

    pub fn add_entry(&mut self, service: &str, username: &str, password: &str) -> Result<()> {
        self.vault_mut()?.add(PasswordEntry::new(service, username, password));
        self.needs_update = true;
        Ok(())
    }

    pub fn get_entry(&mut self, service: Option<String>, username: Option<String>) -> Result<Vec<&PasswordEntry>> {
        let res = self.vault_mut()?.get(service, username);
        Ok(res)
    }

    pub fn delete_entry(&mut self, service: &str, username: &str) -> Result<()> {
        self.vault_mut()?.delete(service, username);
        self.needs_update = true;
        Ok(())
    }

    pub fn check_vault_file(&mut self) -> Result<()> {
        self.serial.write("CHECK_VAULT_FILE\n")?;
        let response = self.serial.read_line()?;

        self.is_init = match response.trim() {
            "VAULT_EXISTS" => true,
            "VAULT_NOT_EXISTS" => false,
            res => return Err(anyhow!("Invalid arduino response: {:?}", res))
        };

        Ok(())
    }

    pub fn unlock(&mut self, password: &str) -> Result<()> {
        // get salt and derive master key
        let salt = self.serial.get_salt()?;
        let master_key = crypto::dervive_key(password, &salt)?;

        // get vault from Arduino
        self.serial.write("GET_VAULT\n")?;
        
        // read line none:ciphertext:auth_tag (hex)
        let line = self.serial.read_line()?;

        // split into fields
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() != 3 {
            return Err(anyhow!("Invalid Arduino response: {:?}", line))
        }
        let nonce = hex::decode(parts[0])?;
        let ciphertext = hex::decode(parts[1])?;
        let auth_tag = hex::decode(parts[2])?;

        // decrypt vault
        let plaintext = crypto::decrypt_data(&master_key, &nonce, &ciphertext, &auth_tag)?;

        // parse vault
        let password_vault: PasswordVault = serde_json::from_slice(&plaintext)
            .map_err(|e| anyhow!("Failed to parse vault data: {:?}", e.to_string()))?;

        // update state
        self.master_key = Some(Zeroizing::new(master_key));
        self.vault = Some(Zeroizing::new(password_vault));
        self.is_locked = false;

        Ok(())
    }

    pub fn update_vault_file(&mut self) -> Result<()> {
        if !self.needs_update {
            return Ok(())
        }

        // encrypt vault
        let password_vault_json = serde_json::to_string(self.vault_mut()?)?;
        let (nonce, ciphertext, auth_tag) = crypto::encrypt_data(self.master_key()?, password_vault_json.as_bytes())?;

        // send encrypted vault to arduino (hex)
        let init_vault_command = format!(
            "UPDATE_VAULT:{}:{}:{}\n",
            hex::encode(nonce),
            hex::encode(ciphertext),
            hex::encode(auth_tag)
        );
        self.serial.write(&init_vault_command)?;        

        Ok(())
    }

    fn vault_mut(&mut self) -> Result<&mut PasswordVault> {
        self.vault
            .as_deref_mut()
            .ok_or_else(|| anyhow!("Vault is not available!"))
    }

    fn master_key(&self) -> Result<&[u8; MASTER_KEY_LEN]> {
        self.master_key
            .as_deref()
            .ok_or_else(|| anyhow!("Master key is not available!"))
    }
}