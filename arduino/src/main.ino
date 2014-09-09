#define LED_PIN 13
#define SERIAL_MAX_READ_BYTES 80


char line_from_serial[SERIAL_MAX_READ_BYTES];
char status;


// Read a line from serial until the newline character is reached.
// Data is written into the ``line_from_serial`` variable.
void readLine() {
    int nchars = 0;
    while (nchars == 0) {
        nchars = Serial.readBytesUntil('\n', line_from_serial, SERIAL_MAX_READ_BYTES);
    }
}


// Setup arduino.
void setup() {
    // Initialize serial connection
    Serial.setTimeout(500);
    Serial.begin(19200);

    // Set output pins
    pinMode(LED_PIN, OUTPUT);
}


// Arduino main loop.
void loop() {
    readLine();
    status = line_from_serial[0];

    if (status == '0') {
        digitalWrite(LED_PIN, LOW);
    } else if (status == '1') {
        digitalWrite(LED_PIN, HIGH);
    }
}
