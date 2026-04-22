use cli_utils::colors::*;
use cli_utils::config::{Logging, LogLevel, LogOutput};

fn main() {

    const DIVIDER: &str = "--------------------------";

    // Testing Colors
    println!("{DIVIDER}");
    println!("{}", red("This text uses the red function."));
    let mut red_text = ColorString {
        color: Color::Red,
        string: String::from("This text uses the ColorString struct."),
        colorized: String::new(),
    };

    ColorString::paint(&mut red_text);
    println!("{}", red_text.colorized);

    // Testing Config
    println!("{DIVIDER}");
    let config = Logging { 
        enabled: true, 
        level: LogLevel::Info, 
        destination: LogOutput::File("config-testing.log".to_string())
    };

    let log_file = match config.destination {
        LogOutput::File(path) => path,
        _ => panic!("This shouldn't happen!"),
        };

    //println!("{}", log_file);
    let result = Logging::test_file(&log_file);
    match result {
        Ok(()) => println!("{}", green("Created new logging file.")),
        Err(e) => println!("{} {}", red("Failed to create logging file:"), e),
    }
}