#include <SPI.h>
#include <SD.h>

const char* VAULT_FILE    = "vault.bin";
const char* SALT_FILE     = "salt.bin";
const char* NONCE_FILE    = "nonce.bin";
const char* AUTH_TAG_FILE = "auth_tag.bin";

const int SALT_LEN        = 16;
const int NONCE_LEN       = 12;
const int AUTH_TAG_LEN    = 16;

void setup() {
    Serial.begin(115200);
    while (!Serial);

    pinMode(SDCARD_SS_PIN, OUTPUT);
    digitalWrite(SDCARD_SS_PIN, HIGH);
    if (!SD.begin()) {
        Serial.println("SD card initialization failed.");
        while (true);
    }
}

void loop() {
    if (Serial.available()) {
        String header = Serial.readStringUntil('\n');
        if (header.startsWith("UPDATE_SALT:")) {
            handleUpdateSalt(header);
        }
        else if (header.startsWith("UPDATE_VAULT:")) {
            handleUpdateVault(header);
        }
        else if (header == "GET_SALT") {
            handleGetSalt();
        }
        else if (header == "GET_VAULT") {
            handleGetVault();
        }
        else if (header == "CHECK_VAULT_FILE") {
            handleCheckVaultFile();
        }
        else if (header == "RESET_VAULT") {
            handleResetVault();
        }
        else {
            Serial.print("Invalid header: ");
            Serial.println(header);
        }
    }
}

void handleUpdateSalt(String header) {
    int len = header.substring(strlen("UPDATE_SALT:")).toInt();
    if (len != SALT_LEN) {
        Serial.println("Invalid salt length.");
        return;
    }
    eraseIfExists(SALT_FILE);
    writeBinToFile(SALT_FILE, len);
}

void handleUpdateVault(String header) {
    int len = header.substring(strlen("UPDATE_VAULT:")).toInt();
    if (len < NONCE_LEN + AUTH_TAG_LEN) {
        Serial.println("Invalid vault payload lenght.");
        while (len--) {
            Serial.read();
        }
        return;
    }

    eraseIfExists(NONCE_FILE);
    eraseIfExists(VAULT_FILE);
    eraseIfExists(AUTH_TAG_FILE);

    writeBinToFile(NONCE_FILE, NONCE_LEN);
    int cipherLen = len - NONCE_LEN - AUTH_TAG_LEN;
    writeBinToFile(VAULT_FILE, cipherLen);
    writeBinToFile(AUTH_TAG_FILE, AUTH_TAG_LEN);
}

void handleGetSalt() {
    File file = SD.open(SALT_FILE);
    if (!file) {
        Serial.print("Error opening file: ");
        Serial.println(SALT_FILE);
        return;
    }
    size_t len = file.size();
    file.close();
    
    Serial.print("SALT");
    Serial.print(':');
    Serial.print(len);
    Serial.print('\n');
    sendBinFile(SALT_FILE);
}

void handleGetVault() {
    File vaultFile = SD.open(VAULT_FILE, FILE_READ);
    if (!vaultFile) {
        Serial.print("Error opening file: ");
        Serial.println(VAULT_FILE);
        return;
    }
    size_t vaultSize = vaultFile.size();
    vaultFile.close();
    size_t totalSize = NONCE_LEN + vaultSize + AUTH_TAG_LEN;

    Serial.print("VAULT:");
    Serial.print(totalSize);
    Serial.print('\n');

    sendBinFile(NONCE_FILE);
    sendBinFile(VAULT_FILE);
    sendBinFile(AUTH_TAG_FILE);
}

void handleCheckVaultFile() {
    if (SD.exists(VAULT_FILE)) {
        Serial.println("VAULT_EXISTS");
    } else {
        Serial.println("VAULT_NOT_EXISTS");
    }
}

void handleResetVault() {
    eraseIfExists(SALT_FILE);
    eraseIfExists(NONCE_FILE);
    eraseIfExists(VAULT_FILE);
    eraseIfExists(AUTH_TAG_FILE);
    Serial.println("RESET_OK");
}

void eraseIfExists(const char* path) {
    if (SD.exists(path)) {
        SD.remove(path);
    }
}

void writeBinToFile(const char* path, size_t len) {
    File file = SD.open(path, FILE_WRITE);
    if (!file) {
        Serial.print("Error writing to file: ");
        Serial.println(path);
        while (len--) {
            Serial.read();
        }
        return;
    }

    size_t toRead = len;
    uint8_t buffer[32];
    while (toRead > 0) {
        size_t chunk = min(sizeof(buffer), toRead);
        Serial.readBytes(buffer, chunk);
        file.write(buffer, chunk);
        toRead -= chunk;
    }
    file.close();
}

void sendHeader(const char* path, const char* label) {
    
}

void sendBinFile(const char* path) {
    File file = SD.open(path);
    if (!file) {
        Serial.print("Error opening file: ");
        Serial.println(path);
        return;
    }

    uint8_t buffer[32];
    while (file.available()) {
        size_t read = file.read(buffer, min(sizeof(buffer), file.available()));
        Serial.write(buffer, read);
    }
    file.close();
}