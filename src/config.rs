//! This module contains the configuration options for the application.
//! # Examples:
//! ```
//! use cli_utils::config::Logging;
//! let config = Logging::new();
//! ```
//! 
use std::fs::File;
use std::io::Write;

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
/// let config = Logging{ enabled: true, level: LogLevel::Info, destination: LogOutput::Stdout };
/// ```

// Making the struct pub isn't enough. You have to expicitly make all fields public
pub struct Logging {
    pub enabled: bool,
    pub level: LogLevel,
    pub destination: LogOutput,   
}

impl Logging {
    pub fn new() -> Self {
        Self {
            enabled: false,
            level: LogLevel::Info,
            destination: LogOutput::Stdout,
        }
    }

    pub fn test_file(f: &str) -> std::io::Result<()> {
        // I can't use File::create_new() because this uses Rust 2021
        let file = File::options().read(true).write(true).create_new(true).open(f)?;
        Ok(())
    }
}