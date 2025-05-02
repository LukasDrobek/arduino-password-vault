use std::io::{self, BufRead, BufReader, Write};
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key
};
use argon2::Argon2;
use std::time::Duration;

pub fn test_connection() -> io::Result<()> {
    let ports = serialport::available_ports()?;
    let port_name = ports
        .into_iter()
        .find(|p| p.port_name.contains("ttyACM") || p.port_name.contains("ttyUSB"))
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Arduino not found"))?
        .port_name;

    println!("Using port {}", port_name);

    let mut port = serialport::new(port_name, 115200)
        .timeout(Duration::from_millis(1000))
        .open()?;

    let mut reader = BufReader::new(port.try_clone()?);
    loop {
        print!("Enter an integer (or 'q' to quit): ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input == "q" {
            println!("Shutting down...");
            break;
        }

        port.write_all(input.as_bytes())?;
        port.write_all(b"\n")?;
        port.flush()?;

        let mut response = String::new();
        reader.read_line(&mut response)?;
        let num: i32 = match response.trim().parse() {
            Ok(n) => n,
            Err(e) => {
                eprintln!("Error parsing response {}", e);
                continue;
            }
        };

        println!("Arduino reponse: {}", num);
    }

    Ok(())
}

pub fn test_argon() -> io::Result<()> {
    let argon2 = Argon2::default();
    let password = b"net123";
    // random salt (saved on SD card in the future)
    let salt = b"some salt";
    
    print!("Please enter your password: ");
    io::stdout().flush()?;
    let mut input_password = String::new();
    std::io::stdin().read_line(&mut input_password)?;
    let input_password = input_password.trim();
    
    let mut master_key = [0u8; 32];
    let mut input_master_key = [0u8; 32];

    // generate master key (saved on ATECC608 in the future)
    argon2
        .hash_password_into(password, salt, &mut master_key)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    // derive key from user input
    argon2
        .hash_password_into(input_password.as_bytes(), salt, &mut input_master_key)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    if master_key == input_master_key {
        println!("> OK: KEYS MATCH!");
    }
    else {
        println!("> ERROR: KEYS DO NOT MATCH!");
    }
    println!("Master key: {:02x?}", master_key);
    println!("Derived key: {:02x?}", input_master_key);

    Ok(())
}

pub fn test_aes256gcm() -> io::Result<()> {
    let derived_key= b"some_derived_key_123456789012345";
    let key = Key::<Aes256Gcm>::from_slice(derived_key);
    let cipher = Aes256Gcm::new(&key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let cipher_text = cipher
        .encrypt(&nonce, b"plaintext message".as_ref())
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    let plain_text = cipher
        .decrypt(&nonce, cipher_text.as_ref())
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    let plain_text_str = String::from_utf8(plain_text)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;

    println!("Decrypted text: {:?}", plain_text_str);

    Ok(())
}