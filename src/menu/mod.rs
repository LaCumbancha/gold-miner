mod utils;

use std::io;
use std::io::Write;
use std::num::ParseIntError;
use crate::model::system::System;
use crate::utils::logger::LoggerLevel;
use crate::utils::logger::LoggerLevel::DEBUG;

static DEFAULT_ZONES: i32 = 5;
static DEFAULT_MINERS: i32 = 5;
static DEFAULT_LOGGER: LoggerLevel = DEBUG;

pub fn run() {
    let mut exit_option: bool = false;
    let mut zones: i32 = DEFAULT_ZONES;
    let mut miners: i32 = DEFAULT_MINERS;
    let mut logger: LoggerLevel = DEFAULT_LOGGER;

    display_main_menu();
    while !exit_option {
        let option: Result<i32, ParseIntError> = utils::read_integer();
        match option {
            Ok(value) => {
                match value {
                    0 => {
                        exit_option = true;
                        println!("Thanks for playing Gold Miner!");
                    }
                    1 => {
                        System::start(miners, zones, logger);
                        display_main_menu();
                    }
                    2 => {
                        let (new_zones, new_miners, new_logger) = settings_menu(zones, miners, logger);
                        zones = new_zones;
                        miners = new_miners;
                        logger = new_logger;
                        display_main_menu();
                    }
                    _ => {
                        print!("Wrong option! Retry: ");
                        io::stdout().flush().expect("Error flushing stdout.");
                    }
                }
            }
            Err(..) => {
                print!("Wrong option! Retry: ");
                io::stdout().flush().expect("Error flushing stdout.");
            }
        }
    }
}

pub fn display_main_menu() {
    println!();
    println!("Gold Miner Menu!");
    println!("Select your option:");
    println!("[1] Run system");
    println!("[2] Settings");
    print!("Selection: [0 for quit] ");
    io::stdout().flush().expect("Error flushing stdout.");
}

fn settings_menu(mut zones: i32, mut miners: i32, mut logger: LoggerLevel) -> (i32, i32, LoggerLevel) {
    let mut exit_option: bool = false;

    display_settings_menu(zones, miners, logger);
    while !exit_option {
        let option: Result<i32, ParseIntError> = utils::read_integer();
        match option {
            Ok(value) => {
                match value {
                    0 => {
                        exit_option = true;
                    }
                    1 => {
                        print!("Insert new miners: ");
                        io::stdout().flush().expect("Error flushing stdout.");
                        let new_miners: Result<i32, ParseIntError> = utils::read_integer();
                        if new_miners.is_ok() {
                            miners = new_miners.unwrap()
                        } else {
                            println!("Invalid option!");
                        }
                        display_settings_menu(zones, miners, logger);
                    }
                    2 => {
                        print!("Insert new zones: ");
                        io::stdout().flush().expect("Error flushing stdout.");
                        let new_zones: Result<i32, ParseIntError> = utils::read_integer();
                        if new_zones.is_ok() {
                            zones = new_zones.unwrap()
                        } else {
                            println!("Invalid option!");
                        }
                        display_settings_menu(zones, miners, logger);
                    }
                    3 => {
                        print!("Select new debug level: [1] DEBUG, [2] INFO, [3] ERROR ");
                        io::stdout().flush().expect("Error flushing stdout.");
                        let new_logger: Result<i32, ParseIntError> = utils::read_integer();
                        if new_logger.is_ok() {
                            let new_logger_value = new_logger.unwrap();
                            if (1..4).contains(&new_logger_value) {
                                logger = LoggerLevel::from_i32(new_logger_value);
                            } else {
                                println!("Invalid option!");
                            }
                        } else {
                            println!("Invalid option!");
                        }
                        display_settings_menu(zones, miners, logger);
                    }
                    _ => {
                        print!("Wrong option! Retry: ");
                        io::stdout().flush().expect("Error flushing stdout.");
                    }
                }
            }
            Err(..) => {
                print!("Wrong option! Retry: ");
                io::stdout().flush().expect("Error flushing stdout.");
            }
        }
    }

    return (zones, miners, logger);
}

fn display_settings_menu(zones: i32, miners: i32, logger: LoggerLevel) {
    println!();
    println!("Current settings:");
    println!("[1] Miners: {}", miners);
    println!("[2] Zones: {}", zones);
    println!("[3] Logger level: {}", logger);
    print!("What do you want to change? [0 for quit] ");
    io::stdout().flush().expect("Error flushing stdout.");
}
