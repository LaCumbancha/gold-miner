use std::{fs, io};
use std::fs::{File, OpenOptions};
use std::sync::mpsc::Receiver;
use std::io::Write;

use crate::utils::utils::TimeLogged;
use chrono::{Utc, Datelike};

static LOGS_FOLDER: &str = "./logs";

pub struct Logger {
    file: File,
    receiver: Receiver<String>
}

impl Logger {

    pub fn new(receiver: Receiver<String>) -> io::Result<Logger> {
        fs::create_dir_all(LOGS_FOLDER)?;
        let file = OpenOptions::new().read(true).write(true).create(true).open(Logger::log_name())?;

        Ok(Logger { file, receiver })
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
            self.file.write(received.time_logged().as_bytes());
        }
    }

}
