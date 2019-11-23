use std::io;
use std::io::Write;
use std::sync::mpsc::Sender;
use std::collections::HashMap;

extern crate rand;

use rand::Rng;
use rand::prelude::ThreadRng;

use crate::model::utils::CheckedSend;
use crate::model::miner::Miner;
use crate::model::map::MapSection;
use crate::model::communication::MiningMessage;
use crate::model::communication::MiningMessage::*;

pub type MinerId = i32;

pub struct Foreman {
    miners: Vec<Miner>,
    sections: Vec<MapSection>,
    miners_channels: HashMap<MinerId, Sender<MiningMessage>>,
}

impl Foreman {
    pub fn new(sections: i32) -> Foreman {
        // Generating sections randomly.
        let mut random_generator: ThreadRng = rand::thread_rng();
        let mut region_sections: Vec<MapSection> = Vec::new();
        for section_id in 1..=sections {
            region_sections.push((section_id, random_generator.gen_range(0.0, 1.0)));
        }

        Foreman {
            miners: Vec::new(),
            sections: region_sections,
            miners_channels: HashMap::new(),
        }
    }

    pub fn hire_miners(&mut self, miners: i32) {
        for id in 1..=miners {
            let miner: Miner = Miner::new(id);
            let (miner_id, sending_channel): (i32, Sender<MiningMessage>) = miner.contact();
            // TODO: Make miners work in different threads.
            self.miners.push(miner);
            self.miners_channels.insert(miner_id, sending_channel);
        }
    }

    pub fn start_mining(&self) {
        for section in &self.sections {
            println!("Yo' filthy rats! Go find me some gold in Section {}", section.1);

            // TODO: Check errors when sending messages.
            self.miners_channels.values()
                .for_each(|miner_channel| miner_channel.send(Start(*section)).check_sending());

            self.wait();

            // TODO: Check errors when sending message.
            self.miners_channels.values()
                .map(|miner_channel| miner_channel.send(Stop).check_sending());
        }
    }

    fn wait(&self) {
        print!("Press [ENTER] to make miners stop digging.");
        io::stdout().flush().expect("Error flushing stdout.");
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("Failed to read from stdin.");
    }

    fn send_callback(miner_id: MinerId, message: MiningMessage) -> &dyn Fn() -> () {
        return {
            println!("Error sending {} to miner {}", message, miner_id);
        }
    }

}
