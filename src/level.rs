
use std::fmt;
use std::fmt::Formatter;
use colored::Colorize;
#[derive(Debug)]
pub enum Level {
    DEBUG,
    INFO,
    WARN,
    ERROR
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Level::DEBUG =>     write!(f, "{}", "DEBUG".bright_white()),
            Level::INFO =>      write!(f, "{}", "INFO".bright_green()),
            Level::WARN =>      write!(f, "{}", "WARN".bright_yellow()),
            Level::ERROR =>     write!(f, "{}", "ERROR".bright_red())
        }
    }
}