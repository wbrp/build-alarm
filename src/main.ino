#define LED_PIN 13

void blink(int duration) {
    digitalWrite(LED_PIN, HIGH);
    delay(duration);
    digitalWrite(LED_PIN, LOW);
    delay(duration);
}

void setup() {
    pinMode(LED_PIN, OUTPUT);
}

void loop() {
    blink(600);
    blink(600);
    for (byte i = 0; i < 3; i++) {
        blink(100);
    }
    delay(500);
}
