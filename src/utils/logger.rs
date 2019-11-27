use chrono::{Utc, Datelike};
use std::{fs, io};
use std::fs::{File, OpenOptions};
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use std::io::Write;

use crate::utils::utils::Logging;
use crate::utils::utils::TimeLogged;

static LOGS_FOLDER: &str = "./logs";

pub struct LoggerWriter {
    file: File,
    receiver: Receiver<String>
}

impl LoggerWriter {

    pub fn new(receiver: Receiver<String>) -> io::Result<LoggerWriter> {
        fs::create_dir_all(LOGS_FOLDER)?;
        let file = OpenOptions::new().read(true).write(true).create(true).open(LoggerWriter::log_name())?;

        Ok(LoggerWriter { file, receiver })
    }

    fn log_name() -> String {
        let now = Utc::now();
        let mut buffer: String = String::new();
        buffer.push_str(now.year().to_string().as_str());
        buffer.push_str(now.month().to_string().as_str());
        buffer.push_str(now.day().to_string().as_str());
        buffer.push_str(".log");
        return buffer
    }

    pub fn run(&mut self) {
        for received in self.receiver {
            self.file.write(received.as_bytes());
        }
    }

}

#[derive(Clone)]
pub struct Logger {
    sender: Sender<String>
}

impl Logger {

    pub fn new(sender: Sender<String>) -> Logger {
        Logger { sender }
    }

    pub fn debug(&self, message: String) {
        self.sender.log(format!("[DEBUG] {}", message.time_logged()));
    }

    pub fn info(&self, message: String) {
        self.sender.log(format!("[INFO] {}", message.time_logged()));
    }

    pub fn error(&self, message: String) {
        self.sender.log(format!("[ERROR] {}", message.time_logged()));
    }

}