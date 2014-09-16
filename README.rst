Build Alarm
===========

A small build alarm program to connect Jenkins to an alarm light.

Building
--------

Requirements (on ArchLinux)::

    # Needed for the arduino
    $ sudo pacman -S avrdude avr-gcc avr-binutils avr-libc
    $ yaourt -S arduino

    # Needed for the Makefile
    $ sudo pacman -S make perl-device-serialport perl-yaml

Clone and initialize submodules::

    $ git submodule init
    $ git submodule update

Build and upload::

    $ cd arduino
    $ make upload
