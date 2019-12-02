use std::sync::mpsc::{Receiver, Sender, channel};
use std::thread::JoinHandle;
use std::thread;

use crate::model::foreman::Foreman;
use crate::utils::logger::{LoggerWriter, Logger};

pub struct System {}

impl System {

    pub fn start(miners: i32, zones: i32) {
        println!();

        let (logger_in, logger_out): (Sender<String>, Receiver<String>) = channel();

        let logger_handler: JoinHandle<()> = thread::spawn(move || {
            LoggerWriter::run(logger_out);
        });

        let mut foreman: Foreman = Foreman::new(zones, Logger::new(logger_in));
        foreman.hire_miners(miners);
        foreman.start_mining();

        //logger_handler.join().unwrap();
    }

}