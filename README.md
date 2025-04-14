# Arduino Password Vault

A secure hardware-based password manager.

## Vault Components

- **Interactive CLI App in Rust**
    - Derives keys, encrypts/decrypts data
    - Communicates with Arduino via serial

- **Arduino MKR Zero**
    - Stores encrypted data and HMAC on SD card
    - Hosts the ATECC608A for hardware-backed HMAC and secure key storage

- **ATECC608A Secure Element**
    - Stores HMAC key
    - Performs cryptographic operations

## Usage

1. Connect the Arduino to your computer
2. Run the CLI application
3. Initialize the password
4. Set-up a master password and keep it safe, there is no *"forgot password"* here
5. Now you're all set to add, view or edit your passwords

## Security considerations

- The master password is never stored anywhere
- All cryptographic operations are performed in hardware where possible
- The system is air-gapped, there are no wireless connectivity modules