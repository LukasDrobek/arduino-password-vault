use anyhow::{Result, anyhow};
use rand::RngCore;
use rand::rngs::OsRng;
use zeroize::Zeroizing;

use crate::constants::{AUTH_TAG_LEN, MASTER_KEY_LEN, NONCE_LEN, SALT_LEN};
use crate::crypto;
use crate::serial::SerialManager;
use crate::vault::{PasswordEntry, PasswordVault};

pub struct VaultManager {
    serial: SerialManager,
    master_key: Option<Zeroizing<[u8; MASTER_KEY_LEN]>>,
    vault: Option<Zeroizing<PasswordVault>>,
    is_init: bool,
    is_locked: bool,
    needs_update: bool,
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
            needs_update: false,
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
        let (nonce, ciphertext, auth_tag) =
            crypto::encrypt_data(&master_key, password_vault_json.as_bytes())?;

        // send salt to Arduino (len + raw bytes)
        let salt_header = format!("UPDATE_SALT:{}\n", salt.len());
        self.serial.write_str(&salt_header)?;
        self.serial.write_bytes(&salt)?;

        // send encrypted vault to Arduino (len + raw bytes)
        let mut vault_payload = Vec::new();
        vault_payload.extend_from_slice(&nonce);
        vault_payload.extend_from_slice(&ciphertext);
        vault_payload.extend_from_slice(&auth_tag);

        let vault_header = format!("UPDATE_VAULT:{}\n", vault_payload.len());
        self.serial.write_str(&vault_header)?;
        self.serial.write_bytes(&vault_payload)?;

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
        self.vault_mut()?
            .add(PasswordEntry::new(service, username, password));
        self.needs_update = true;
        Ok(())
    }

    pub fn get_entry(
        &mut self,
        service: Option<String>,
        username: Option<String>,
    ) -> Result<Vec<&PasswordEntry>> {
        let res = self.vault_mut()?.get(service, username);
        Ok(res)
    }

    pub fn delete_entry(&mut self, service: &str, username: &str) -> Result<()> {
        self.vault_mut()?.delete(service, username);
        self.needs_update = true;
        Ok(())
    }

    pub fn check_vault_file(&mut self) -> Result<()> {
        self.serial.write_str("CHECK_VAULT_FILE\n")?;
        let response = self.serial.read_line()?;

        self.is_init = match response.trim() {
            "VAULT_EXISTS" => true,
            "VAULT_NOT_EXISTS" => false,
            res => return Err(anyhow!("Invalid arduino response: {:?}", res)),
        };

        Ok(())
    }

    pub fn unlock(&mut self, password: &str) -> Result<()> {
        // request salt from Arduino
        self.serial.write_str("GET_SALT\n")?;
        let mut salt = [0u8; SALT_LEN];
        self.serial.read_exact(&mut salt)?;

        // derive master key
        let master_key = crypto::dervive_key(password, &salt)?;

        // request vault from Arduino
        self.serial.write_str("GET_VAULT\n")?;

        // read header
        let header = self.serial.read_line()?;
        let len = header
            .strip_prefix("VAULT:")
            .ok_or_else(|| anyhow!("Bad header: {}", header))?
            .parse::<usize>()?;
        if len < NONCE_LEN + AUTH_TAG_LEN {
            return Err(anyhow!("Invalid vault payload length: {}", len));
        }

        // read raw bytes
        let mut buffer: Vec<u8> = vec![0u8; len];
        self.serial.read_exact(&mut buffer)?;

        // split into fields
        let nonce = buffer[..NONCE_LEN].to_vec();
        let auth_tag = buffer[len - AUTH_TAG_LEN..].to_vec();
        let ciphertext = buffer[NONCE_LEN..len - AUTH_TAG_LEN].to_vec();

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
            return Ok(());
        }

        // encrypt vault
        let password_vault_json = serde_json::to_string(self.vault_mut()?)?;
        let (nonce, ciphertext, auth_tag) =
            crypto::encrypt_data(self.master_key()?, password_vault_json.as_bytes())?;

        // send ecnrypted vault to Arduino (len + raw bytes)
        let mut vault_payload = Vec::new();
        vault_payload.extend_from_slice(&nonce);
        vault_payload.extend_from_slice(&ciphertext);
        vault_payload.extend_from_slice(&auth_tag);

        let vault_header = format!("UPDATE_VAULT:{}\n", vault_payload.len());
        self.serial.write_str(&vault_header)?;
        self.serial.write_bytes(&vault_payload)?;

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
