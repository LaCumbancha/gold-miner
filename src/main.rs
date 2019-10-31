extern crate clap;
use clap::{Arg, App, SubCommand};

mod model;

fn main() {
    model::menu::run();
}
