extern crate serial;
extern crate time;

mod buildlamp;

use std::env;
use std::process;

use time::Duration;
use serial::BaudRate;

use buildlamp::BuildLamp;


/// Return the badge URL or an exit code
fn parse_args() -> Result<String, i32> {

    // Get first argument
    match env::args_os().nth(1) {

        // If an argument is present, convert it to a string
        Some(arg) => arg.into_string().map_err(|_| {
            println!("Invalid badge URL, does it contain non-unicode data?");
            return 1
        }),

        // Otherwise, show usage
        None => {
            let program_name = env::args_os().nth(0).unwrap().into_string().unwrap();
            println!("Usage: {} <jenkins-build-badge-url>", program_name);
            return Err(1)
        }

    }

}


fn main() {

    // Parse arguments
    let badge_url = parse_args().unwrap_or_else(|exit_code| {
        process::exit(exit_code);
    });

    // Initialize build lamp
    let mut lamp = BuildLamp::new(serial::open("/dev/ttyACM0").unwrap());
    lamp.initialize(BaudRate::Baud19200, Duration::seconds(1)).map_err(|_| {
        println!("Could not initialize build lamp.");
        process::exit(2);
    });

    println!("{}", badge_url);
}
