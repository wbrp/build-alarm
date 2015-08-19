use std::io::Write;

use serial::{SerialDevice, SerialPortSettings, BaudRate, Error};
use serial::posix::TTYPort;
use time::Duration;


pub struct BuildLamp {
    pub port: TTYPort,
}


impl BuildLamp {

    /// Return a new BuildLamp instance
    pub fn new(port: TTYPort) -> BuildLamp {
        BuildLamp {
            port: port,
        }
    }

    /// Initialize and configure the serial connection
    pub fn initialize(&mut self, baudrate: BaudRate, timeout: Duration) -> Result<(), Error> {
        // Set the baud rate
        let mut settings = try!(self.port.read_settings());
        try!(settings.set_baud_rate(baudrate));
        try!(self.port.write_settings(&settings));

        // Set the timeout
        try!(self.port.set_timeout(timeout));

        Ok(())
    }

    pub fn turn_on(&mut self) -> Result<(), Error> {
        try!(self.port.write(b"1"));
        try!(self.port.flush());
        Ok(())
    }

    pub fn turn_off(&mut self) -> Result<(), Error> {
        try!(self.port.write(b"0"));
        try!(self.port.flush());
        Ok(())
    }

}
