//! This module contains the configuration options for the application.
//! # Examples:
//! ```
//! use cli_utils::config::Logging;
//! let config = Logging::new();
//! ```
//! 
//use std::io::{Error};
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

// If the enum is pub, then all variants are pub as well. No need for `pub` to preceed the variants.
pub enum LogOutput {
    Stdout,
    Stderr,
    File(String),
}

/// This struct contains configuration options for the application.
/// # Examples:
/// ```
/// use cli_utils::config::Logging;
/// let config = Logging::new();
/// ```
/// 
/// Creating a new instance of the Logging struct:
/// ```
/// use cli_utils::config::{Logging, LogLevel, LogOutput};
/// let mut config = Logging::new();
/// config.set_enabled(true);
/// config.set_level(LogLevel::Debug);
/// config.set_destination(LogOutput::Stderr);
/// ```

// Making the struct pub isn't enough. You have to expicitly make all fields public
pub struct Logging {
    enabled: bool,
    level: LogLevel,
    destination: LogOutput,   
}

impl Logging {
    pub fn new() -> Self {
        Self {
            enabled: false,
            level: LogLevel::Info,
            destination: LogOutput::Stdout,
        }
    }

    pub fn set_enabled(&mut self, e: bool) -> Result<(), String> {
        self.enabled = e;
        Ok(())
    }

    pub fn set_level(&mut self, l: LogLevel) -> Result<(), String> {
        self.level = l;
        Ok(())
    }

    pub fn set_destination(&mut self, output: LogOutput) -> Result<(), std::io::Error> {
        match output {
            LogOutput::Stdout => { 
                self.destination = LogOutput::Stdout; 
                Ok(()) },
            LogOutput::Stderr => { 
                self.destination = LogOutput::Stderr; 
                Ok(()) },
            LogOutput::File(s) => {
                match Logging::create_log_file(&s) {
                    Ok(_) => { 
                        self.destination = LogOutput::File(s);
                        Ok(()) },
                    Err(e) => Err(e),
                }
            }
        }
    }

    fn create_log_file(s: &str) -> Result<(), std::io::Error> {
        use std::fs::File;
        File::create_new(s)?;
        Ok(())
    }
}