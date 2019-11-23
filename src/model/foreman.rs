use std::io;
use std::io::Write;
use std::sync::mpsc::Sender;
use std::collections::HashMap;

extern crate rand;
use rand::Rng;
use rand::prelude::ThreadRng;

use crate::model::miner::Miner;
use crate::model::map::MapSection;
use crate::model::communication::MiningMessage;
use crate::model::communication::MiningMessage::*;

pub type MinerId = i32;

pub struct Foreman {
    miners: Vec<Miner>,
    sections: Vec<MapSection>,
    miners_channels: HashMap<MinerId, Sender<MiningMessage>>
}

impl Foreman {

    pub fn new(sections: i32) -> Foreman {
        println!("FOREMAN: Welcome to the Gold Camp! I'm the foreman, the man in charge. Hope we finally get some gold.");
        println!("FOREMAN: Today we'll be exploring this {} zones.", sections);

        // Generating sections randomly.
        let mut random_generator: ThreadRng = rand::thread_rng();
        let mut region_sections: Vec<MapSection> = Vec::new();
        for section_id in 1..=sections {
            region_sections.push((section_id, random_generator.gen_range(0.0, 1.0)));
        }

        Foreman {
            miners: Vec::new(),
            sections: region_sections,
            miners_channels: HashMap::new()
        }

    }

    pub fn hire_miners(&mut self, miners: i32) {
        println!("FOREMAN: But first, we need some cheap manpower. We'll go to the town and get the first {} morons that show up.", miners);
        for id in 1..=miners {
            let miner: Miner = Miner::new(id);
            let (miner_id, sending_channel): (i32, Sender<MiningMessage>) = miner.contact();
            // TODO: Make miners work in different threads.
            self.miners.push(miner);
            self.miners_channels.insert(miner_id, sending_channel);
        }
    }

    pub fn start_mining(&self) {
        print!("FOREMAN: Ok, it's showtime. Let's get this shit done. (Press [ENTER] to make miners start digging)");
        self.wait();
        for section in &self.sections {
            println!();
            print!("FOREMAN: Yo' filthy rats! Go find me some gold in Section {}! ", section.0);

            // TODO: Check errors when sending messages.
            self.miners_channels.values()
                .map(|miner_channel| miner_channel.send(Start(*section)));

            print!("(Press [ENTER] to make miners stop digging)");
            self.wait();

            // TODO: Check errors when sending message.
            self.miners_channels.values()
                .map(|miner_channel| miner_channel.send(Stop));
        }
    }

    fn wait(&self) {
        io::stdout().flush().expect("Error flushing stdout.");
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("Failed to read from stdin.");
    }

}
