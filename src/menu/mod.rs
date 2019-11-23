mod utils;

use std::io;
use std::io::Write;
use std::num::ParseIntError;

static DEFAULT_ZONES: u32 = 5;
static DEFAULT_MINERS: u32 = 33;

pub fn run() {
    let mut exit_option: bool = false;
    let mut zones: u32 = DEFAULT_ZONES;
    let mut miners: u32 = DEFAULT_MINERS;

    while !exit_option {
        display_main_menu();
        let option: Result<u32, ParseIntError> = utils::read_integer();
        match option {
            Ok(value) => {
                match value {
                    0 => {
                        exit_option = true;
                        println!("Thanks for playing Gold Miner!");
                    }
                    1 => {
                        println!("Running system with {} miners and {} zones.", miners, zones);
                    },
                    2 => {
                        let (new_zones, new_miners) = settings_menu(zones, miners);
                        zones = new_zones;
                        miners = new_miners;
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

fn settings_menu(mut zones: u32, mut miners: u32) -> (u32, u32) {
    let mut exit_option: bool = false;

    while !exit_option {
        display_settings_menu(zones, miners);
        let option: Result<u32, ParseIntError> = utils::read_integer();
        match option {
            Ok(value) => {
                match value {
                    0 => {
                        exit_option = true;
                    }
                    1 => {
                        print!("Insert new miners: ");
                        io::stdout().flush().expect("Error flushing stdout.");
                        let new_miners: Result<u32, ParseIntError> = utils::read_integer();
                        if new_miners.is_ok() {
                            miners = new_miners.unwrap()
                        } else {
                            println!("Invalid option!");
                        }
                    },
                    2 => {
                        print!("Insert new zones: ");
                        io::stdout().flush().expect("Error flushing stdout.");
                        let new_zones: Result<u32, ParseIntError> = utils::read_integer();
                        if new_zones.is_ok() {
                            zones = new_zones.unwrap()
                        } else {
                            println!("Invalid option!");
                        }
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

fn display_settings_menu(zones: u32, miners:u32) {
    println!();
    println!("Current settings:");
    println!("[1] Miners: {}", miners);
    println!("[2] Zones: {}", zones);
    print!("What do you want to change? [0 for quit] ");
    io::stdout().flush().expect("Error flushing stdout.");
}
