#!/usr/bin/env python3
import ast
import sys
import time
import urllib.request

import serial


class BuildLamp:
    def __init__(self, device='/dev/ttyACM0', baudrate=19200, timeout=1):
        """
        Initialize Arduino device.
        """
        self.ser = serial.Serial(device, baudrate, timeout=timeout)

    def enable(self):
        """
        Turn on lamp.
        """
        self.ser.write(b'1\n')

    def disable(self):
        """
        Turn off lamp.
        """
        self.ser.write(b'0\n')

    def close(self):
        self.ser.close()


def get_failing_tests(api_url):
    # Do API request
    response = urllib.request.urlopen(api_url)
    content = response.read().decode('utf8')
    data = ast.literal_eval(content)

    # Process data
    jobs = data['jobs']
    tests = [j for j in jobs if j['name'].startswith('test_')]
    failing = [t for t in tests if t['color'] == 'red']

    # Return list of failing tests
    return failing


if __name__ == '__main__':
    # Parse arguments
    if len(sys.argv) != 2:
        print('Usage: %s <jenkins-python-api-url>' % sys.argv[0])
        sys.exit(1)

    # Initialize lamp
    print('Initializing...')
    lamp = BuildLamp()

    # Main event loop
    print('Ready.')
    try:
        while True:
            failing_tests = get_failing_tests(sys.argv[1])
            if len(failing_tests) > 0:
                lamp.enable()
            else:
                lamp.disable()
            time.sleep(3)
    except KeyboardInterrupt:
        lamp.disable()
        lamp.close()
        print('\nGoodbye!')
