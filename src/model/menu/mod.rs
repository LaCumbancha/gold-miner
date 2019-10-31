mod utils;

use std::io;
use std::io::Write;
use std::num::ParseIntError;

pub fn run() {
    display();
    let mut exit_option: bool = false;

    while !exit_option {
        let mut option: Result<u32, ParseIntError> = utils::read_integer();
        match option {
            Ok(value) => {
                match value {
                    0 => {
                        exit_option = true;
                    }
                    1 => {
                        println!("Running system.");
                        println!();
                    },
                    2 => {
                        println!("Settings menu.");
                        println!();
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

pub fn display() {
    println!("Gold Miner Menu!");
    println!("Select your option:");
    println!("[1] Run system");
    println!("[2] Settings");
    print!("Selection: [0 for quit] ");
    io::stdout().flush();
}
