extern crate clap;
use clap::{Arg, App, SubCommand};

mod menu;
mod model;

fn main() {
    menu::run();
    model::miner::Miner::new(0);
}
