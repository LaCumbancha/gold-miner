extern crate clap;
#[macro_use]
extern crate serde_derive;

mod menu;
mod model;
mod utils;

fn main() {
    menu::run();
}
