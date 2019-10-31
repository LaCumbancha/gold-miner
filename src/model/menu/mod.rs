mod utils;

use std::io;
use std::io::Write;
use std::num::ParseIntError;

pub fn run() {
    let mut exit_option: bool = false;

    while !exit_option {
        display_main_menu();
        let option: Result<u32, ParseIntError> = utils::read_integer();
        match option {
            Ok(value) => {
                match value {
                    0 => {
                        exit_option = true;
                    }
                    1 => {
                        println!("Running system.");
                    },
                    2 => {
                        settings_menu();
                    },
                    _ => {
                        print!("Wrong option! Retry: ");
                        io::stdout().flush();
                    }
                }
            },
            Err(..) => {
                print!("Wrong option! Retry: ");
                io::stdout().flush();
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
    io::stdout().flush();
}

fn settings_menu() {
    let mut exit_option: bool = false;

    while !exit_option {
        display_settings_menu();
        let option: Result<u32, ParseIntError> = utils::read_integer();
        match option {
            Ok(value) => {
                match value {
                    0 => {
                        exit_option = true;
                    }
                    1 => {
                        println!("Modifying miners.");
                    },
                    2 => {
                        println!("Modifying zones.");
                    },
                    _ => {
                        print!("Wrong option! Retry: ");
                        io::stdout().flush();
                    }
                }
            },
            Err(..) => {
                print!("Wrong option! Retry: ");
                io::stdout().flush();
            }
        }
    }
}

fn display_settings_menu() {
    println!();
    println!("Current settings:");
    println!("[1] Miners: 0");
    println!("[2] Zones: 0");
    print!("What do you want to change? [0 for quit] ");
    io::stdout().flush();
}
