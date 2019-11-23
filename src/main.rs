extern crate clap;

mod menu;
mod model;

fn main() {
    menu::run();
    model::miner::Miner::new(0);
}
