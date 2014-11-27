#!/usr/bin/env python3
import sys
import time
import urllib.request

import serial


SLEEP_DURATION = 5


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


def test_is_failing(build_badge_url):
    """
    Return whether the test with the specified build badge is failing.
    """

    # Fetch build badge
    response = urllib.request.urlopen(build_badge_url)

    # Check ETag header
    etag = response.headers.get('ETag')

    # Return whether the test failed
    if 'failing' in etag:
        return True
    elif 'passing' in etag:
        return False
    else:
        raise ValueError('Unknown ETag: %s' % etag)


if __name__ == '__main__':
    # Parse arguments
    if len(sys.argv) != 2:
        print('Usage: %s <jenkins-build-badge-url>' % sys.argv[0])
        sys.exit(1)

    # Initialize lamp
    print('Initializing...')
    lamp = BuildLamp()

    # Main event loop
    print('Ready.')
    try:
        while True:
            failing = test_is_failing(sys.argv[1])
            if failing:
                print('Test is failing.')
                lamp.enable()
            else:
                print('Test is passing.')
                lamp.disable()
            time.sleep(SLEEP_DURATION)
    except KeyboardInterrupt:
        lamp.disable()
        lamp.close()
        print('\nGoodbye!')
