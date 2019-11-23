mod utils;

use std::io;
use std::io::Write;
use std::num::ParseIntError;
use crate::model::system::System;

static DEFAULT_ZONES: i32 = 5;
static DEFAULT_MINERS: i32 = 33;

pub fn run() {
    let mut exit_option: bool = false;
    let mut zones: i32 = DEFAULT_ZONES;
    let mut miners: i32 = DEFAULT_MINERS;

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
                        System::start(miners, zones);
                        display_main_menu();
                    },
                    2 => {
                        let (new_zones, new_miners) = settings_menu(zones, miners);
                        zones = new_zones;
                        miners = new_miners;
                        display_main_menu();
                    },
                    _ => {
                        print!("Wrong option! Retry: ");
                        io::stdout().flush().expect("Error flushing stdout.");
                    }
                }
            },
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

fn settings_menu(mut zones: i32, mut miners: i32) -> (i32, i32) {
    let mut exit_option: bool = false;

    display_settings_menu(zones, miners);
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
                        display_settings_menu(zones, miners);
                    },
                    2 => {
                        print!("Insert new zones: ");
                        io::stdout().flush().expect("Error flushing stdout.");
                        let new_zones: Result<i32, ParseIntError> = utils::read_integer();
                        if new_zones.is_ok() {
                            zones = new_zones.unwrap()
                        } else {
                            println!("Invalid option!");
                        }
                        display_settings_menu(zones, miners);
                    },
                    _ => {
                        print!("Wrong option! Retry: ");
                        io::stdout().flush().expect("Error flushing stdout.");
                    }
                }
            },
            Err(..) => {
                print!("Wrong option! Retry: ");
                io::stdout().flush().expect("Error flushing stdout.");
            }
        }
    }

    return (zones, miners);
}

fn display_settings_menu(zones: i32, miners:i32) {
    println!();
    println!("Current settings:");
    println!("[1] Miners: {}", miners);
    println!("[2] Zones: {}", zones);
    print!("What do you want to change? [0 for quit] ");
    io::stdout().flush().expect("Error flushing stdout.");
}
