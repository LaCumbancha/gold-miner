use chrono::{Utc, Datelike};
use std::fs;
use std::fs::OpenOptions;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use std::io::{Write, LineWriter};

use crate::utils::extension::Logging;
use crate::utils::extension::TimeLogged;
use core::fmt;

static LOGS_FOLDER: &str = "./logs";

#[derive(Copy, Debug, Clone, PartialEq)]
pub enum LoggerLevel { DEBUG = 1, INFO = 2, ERROR = 3 }

impl fmt::Display for LoggerLevel {
    fn fmt(&self, format: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LoggerLevel::DEBUG => write!(format, "DEBUG"),
            LoggerLevel::INFO => write!(format, "INFO"),
            LoggerLevel::ERROR => write!(format, "ERROR")
        }
    }
}

impl LoggerLevel {
    pub(crate) fn from_i32(value: i32) -> LoggerLevel {
        match value {
            1 => LoggerLevel::DEBUG,
            2 => LoggerLevel::INFO,
            3 => LoggerLevel::ERROR,
            _ => panic!("Unknown value: {}", value)
        }
    }
}

pub struct LoggerWriter {}

impl LoggerWriter {
    pub fn run(receiver: Receiver<String>) {
        fs::create_dir_all(LOGS_FOLDER).expect("Couldn't create log folder!");
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(LoggerWriter::log_name())
            .expect("Couldn't create log file!");
        let mut line_writer = LineWriter::new(file);

        for received in receiver {
            line_writer.write_all(received.as_bytes()).expect("Error writing log!");
        }
    }

    fn log_name() -> String {
        let now = Utc::now();
        let mut buffer: String = String::new();
        buffer.push_str(LOGS_FOLDER);
        buffer.push_str("/");
        buffer.push_str(now.year().to_string().as_str());

        if now.month() < 10 {
            buffer.push_str("0");
            buffer.push_str(now.month().to_string().as_str());
        } else {
            buffer.push_str(now.month().to_string().as_str());
        }

        if now.day() < 10 {
            buffer.push_str("0");
            buffer.push_str(now.day().to_string().as_str());
        } else {
            buffer.push_str(now.day().to_string().as_str());
        }

        buffer.push_str(".log");
        return buffer;
    }
}

#[derive(Clone)]
pub struct Logger {
    sender: Sender<String>,
    logger_level: LoggerLevel,
}

impl Logger {
    pub fn new(sender: Sender<String>, logger_level: LoggerLevel) -> Logger {
        Logger { sender, logger_level }
    }

    pub fn debug(&self, message: String) {
        if self.logger_level == LoggerLevel::DEBUG {
            self.sender.log(format!("[DEBUG] {}\n", message.time_logged()));
        }
    }

    pub fn info(&self, message: String) {
        if self.logger_level != LoggerLevel::ERROR {
            self.sender.log(format!("[INFO] {}\n", message.time_logged()));
        }
    }

    pub fn error(&self, message: String) {
        self.sender.log(format!("[ERROR] {}\n", message.time_logged()));
    }
}