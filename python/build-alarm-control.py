#!/usr/bin/env python3
import serial
import time


class Arduino:
    def __init__(self, device='/dev/ttyACM0', baudrate=19200, timeout=1):
        """
        Initialize Arduino device.
        """
        self.ser = serial.Serial(device, baudrate, timeout=timeout)

    def blink(self, delay):
        """
        Blink ``delay`` seconds.
        """
        self.ser.write(b'1\n')
        time.sleep(delay)
        self.ser.write(b'0\n')
        time.sleep(delay)

    def loop(self):
        while True:
            self.blink(2)

    def close(self):
        self.ser.close()


if __name__ == '__main__':
    ar = Arduino()
    try:
        ar.loop()
    except KeyboardInterrupt:
        ar.close()
        print('\nGoodbye!')
