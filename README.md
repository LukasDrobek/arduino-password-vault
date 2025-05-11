# Arduino Password Vault

A secure, hardware-based password manager built with Rust and Arduino.

## Overview

This project implements a secure password vault on an Arduino MKR Zero with SD card storage and a Rust-based CLI. It relies on the Argon2 algorithm for key derivation and the AES-256-GCM algorithm to encrypt and decrypt password data.

## Features

- **Master Password Protection**: Derives the encryption key via Argon2; the master password is never stored.
- **AES-256-GCM Encryption**: Ensures integrity through authenticated encryption.
- **Encrypted Storage**: Passwords are stored as a single encrypted file on the SD card.
- **Memory Safety**: Sensitive buffers, including keys and passwords, are zeroed out in RAM immediately after use.
- **Custom Serial Protocol**: Header-based communication enables reliable and fast transmission of raw binary data.
- **Future Features**:
  - `mlock` support to lock memory pages and prevent swapping.
  - Parallel processing to improve performance with large password sets.
  - Per-record storage format to minimize data transfer and reduce data-loss risk on unexpected shutdown.
  - Encrypted backup to the host computer for recovery.

## Architecture

### Hardware Components

- **Arduino MKR Zero**:
  - Manages SD card storage and handles serial I/O.
- **Other Arduino Boards (e.g., Uno, Nano)**:
  - Can be used with an external SD card module wired via SPI.
- **SD Card**:
  - Stores the encrypted vault file.
  - An industrial-grade SD card is recommended for better data retention.

### Software Components

- **Rust CLI Application**:
  1. Prompts for master password and derives an AES-256-GCM key via Argon2.
  2. Opens a serial connection to the Arduino.
  3. Exchanges headers and data blocks to perform secure reads and writes.
  4. Zeroes out keys, and sensitive buffers immediately after each operation.

- **Arduino Firmware**:
  - Listens for commands over serial, reads/writes the encrypted file, and forwards raw data blocks.

## Getting Started

### Installation

1. Clone the repository:
```bash
git clone git@github.com:LukasDrobek/arduino-password-vault.git
cd arduino-password-vault
```
2. Build the Rust CLI
```bash
cargo build --release
```
3. Flash the Arduino firmware
```bash
arduino-cli compile --fqbn arduino:avr:mkrzero firmware/
arduino-cli upload --fqbn arduino:avr:mkrzero firmware/
```

### Usage

1. Insert the SD card and connect the Arduino via USB.
2. Run the CLI application:
```bash
./target/release/vault-cli
```
3. Initialize or unlock the vault with your master password.
4. Use commands to add, view, or remove password entries.

### Prerequisites

- Rust (1.60+)
- Arduino CLI or IDE
- Arduino MKR Zero board
- SD card (formatted FAT32)

## Future Improvements

- **Memory Locking**: Integrate `mlock` to prevent sensitive data from being swapped.
- **Parallel Operations**: Enable concurrent reading/decryption for better throughput.
- **Per-Record Storage**: Transition to storing each password as an individual encrypted record.
- **Automated Backups**: Sync encrypted vault files to the host machine for quick recovery.

## Security Considerations

- The master password is never stored or transmitted.
- All cryptographic operations use AES-256-GCM for authenticated encryption.
- Keys, passwords, and buffers are securely erased from memory after use.
- Serial protocol uses headers to specify block type and length, eliminating desynchronization risks.

## License

This project is licensed under the MIT License. See [LICENSE](./LICENSE) for details.