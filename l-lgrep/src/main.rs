use lilgrep::Config;
use std::{env, process};

fn main() {
    // Get the arguments into config struct type
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        // The eprintln! macro sends the error to the standard error stream
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // The unwrap_or_else doesn't fit here because no Ok value
    // is being expected, just an error that may occur
    if let Err(e) = lilgrep::run(config) {
        eprintln!("Application run error: {}", e);
        process::exit(1);
    }

    
}
