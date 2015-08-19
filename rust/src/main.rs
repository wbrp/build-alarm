extern crate serial;
extern crate time;
extern crate hyper;

mod buildlamp;
mod task;

use std::env;
use std::process;

use time::Duration;
use serial::BaudRate;

use buildlamp::BuildLamp;
use task::JenkinsTask;


/// Return the badge URL or an exit code
fn parse_args() -> Result<String, String> {

    // Get first argument
    match env::args_os().nth(1) {

        // If an argument is present, convert it to a string
        Some(arg) => arg.into_string().map_err(|_| "Invalid badge URL, does it contain non-unicode data?".to_string()),

        // Otherwise, show usage
        None => {
            let program_name = env::args_os().nth(0).unwrap().into_string().unwrap();
            let msg = format!("Usage: {} <jenkins-build-badge-url>", program_name);
            Err(msg)
        }

    }

}


fn main() {

    // Parse arguments
    let badge_url = parse_args().unwrap_or_else(|msg| {
        println!("{}", msg);
        process::exit(1);
    });

    // Initialize jenkins task instance
    let task = JenkinsTask::new(&badge_url).unwrap_or_else(|msg| {
        println!("{}", msg);
        process::exit(2);
    });

    // Initialize build lamp
    let mut lamp = BuildLamp::new("/dev/ttyACM0").unwrap_or_else(|msg| {
        println!("{}", msg);
        process::exit(3);
    });
    lamp.initialize(BaudRate::Baud19200, Duration::seconds(1)).map_err(|_| {
        println!("Could not initialize build lamp.");
        process::exit(3);
    });

    // Take action based on result
    match task.is_failing() {
        Ok(true) => {
            println!("Task is failing");
            lamp.turn_on().unwrap_or_else(|msg| println!("Could not turn on build lamp: {}", msg));
        },
        Ok(false) => {
            println!("Task is not failing");
            lamp.turn_off().unwrap_or_else(|msg| println!("Could not turn off build lamp: {}", msg));
        },
        Err(err) => {
            println!("Could not determine task status: {}", err);
            lamp.turn_off().unwrap_or_else(|msg| println!("Could not turn off build lamp: {}", msg));
        },
    }

}
