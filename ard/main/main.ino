#include <SPI.h>
#include <SD.h>

void setup() {
    Serial.begin(115200);
    while (!Serial);
}

void loop() {
    if (Serial.available()) {
        String command = Serial.readStringUntil('\n');
        processCommand(command);
    }
}

void processCommand(String command) {
    int num = command.toInt();
    ++num;
    Serial.println(num);
}