use chrono::{Timelike, Utc};


pub enum LogLevel {
    OK,
    ERROR,
    WARNING,
    FATAL,
    DEBUG,
}

pub fn log<S>(mut level: LogLevel, message: S) -> () where S: Into<String> {
    let log_level = match level {
        LogLevel::OK => 1,
        LogLevel::ERROR => 2,
        LogLevel::WARNING => 3,
        LogLevel::FATAL => 4,
        LogLevel::DEBUG => 5,
        _ => 0
    };
    let text = message.into();
    if log_level == 1 || log_level == 0 {
        println!("\x1b[35m{}\tLevel {}\t\t\x1b[32m{}\x1b[0m", timestamp(), "OK", text);
    } else if log_level == 2 {
        println!("\x1b[41m\x1b[32m{}\t\tLevel {}\t{}\x1b[0m", timestamp(), "ERROR", text);
    } else if log_level == 3 {
        println!("\x1b[35m{}\tLevel {}\t\x1b[33m{}\x1b[0m", timestamp(), "WARNING", text);
    } else if log_level == 4 {
        println!("\x1b[41m\x1b[33m{}\tLevel {}\t\t{}\x1b[0m", timestamp(), "FATAL", text);
    } else if log_level == 5 {
        println!("\x1b[35m{}\tLevel {}\t\t\x1b[34m{}\x1b[0m", timestamp(), "DEBUG", text);
    }
}

pub fn timestamp() -> String {
    let now = Utc::now();

    return format!("{:02}:{:02}:{:02}", now.hour(), now.minute(), now.second());
}