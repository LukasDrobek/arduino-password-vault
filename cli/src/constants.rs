#![allow(dead_code)]
pub const APP_NAME: &str = env!("CARGO_PKG_NAME");
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const APP_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

pub const BAUD_RATE: u32 = 115200;
pub const NONCE_LEN: usize = 12;
pub const AUTH_TAG_LEN: usize = 16;
pub const SALT_LEN: usize = 32;
pub const MASTER_KEY_LEN: usize = 32;