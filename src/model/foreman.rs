use std::{io, thread};
use std::io::Write;
use std::sync::mpsc::{Sender, Receiver, channel};
use std::collections::HashMap;

extern crate rand;

use rand::Rng;
use rand::prelude::ThreadRng;

use crate::utils::utils::Logging;
use crate::utils::utils::CheckedSend;
use crate::model::miner::Miner;
use crate::model::map::MapSection;
use crate::model::communication::MiningMessage;
use crate::model::communication::MiningMessage::*;
use std::thread::JoinHandle;

pub type MinerId = i32;

pub struct Foreman {
    miners: Vec<Miner>,
    sections: Vec<MapSection>,
    miners_channels: HashMap<MinerId, Sender<MiningMessage>>,
    logger_channel: Sender<String>,
    thread_handlers: Vec<JoinHandle<()>>
}

impl Foreman {
    pub fn new(sections: i32, logger: Sender<String>) -> Foreman {
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
            miners_channels: HashMap::new(),
            logger_channel: logger,
            thread_handlers: Vec::new()
        }
    }

    pub fn hire_miners(&mut self, miners: i32) {
        println!("FOREMAN: But first, we need some cheap manpower. We'll go to the town and get the first {} morons that show up.", miners);

        // Creating channels for every miner.
        let mut channels_in: HashMap<MinerId, Sender<MiningMessage>> = HashMap::new();
        let mut channels_out: HashMap<MinerId, Receiver<MiningMessage>> = HashMap::new();
        for id in 1..=miners {
            let (channel_in, channel_out): (Sender<MiningMessage>, Receiver<MiningMessage>) = channel();
            self.miners_channels.insert(id, channel_in.clone());
            channels_in.insert(id, channel_in);
            channels_out.insert(id, channel_out);
        }

        for id in 1..=miners {
            // Preparing channels for each miner.
            let miner_receiving_channel = channels_out.remove(&id).unwrap();
            let mut miner_adjacent_channels = channels_in.clone();
            miner_adjacent_channels.remove(&id);
            let mut miner_logger = self.logger_channel.clone();

            self.logger_channel.log(format!("Creating miner {}", id));
            let handler: JoinHandle<()> = thread::spawn(move || {
                let mut miner: Miner = Miner::new(id, miner_receiving_channel, miner_adjacent_channels, miner_logger);
                miner.work();
            });

            self.thread_handlers.push(handler);
        }
    }

    pub fn start_mining(&self) {
        print!("FOREMAN: Ok, it's showtime. Let's get this shit done. (Press [ENTER] to make miners start digging)");
        self.wait();
        for section in &self.sections {
            println!();
            print!("FOREMAN: Yo' filthy rats! Go find me some gold in Section {}! ", section.0);

            self.miners_channels.iter()
                .for_each(|(id, channel)|
                    channel.checked_send(
                        Start(*section),
                        Foreman::send_callback(*id)
                    )
                );

            print!("(Press [ENTER] to make miners stop digging)");
            self.wait();

            self.miners_channels.iter()
                .for_each(|(id, channel)|
                    channel.checked_send(
                        Stop,
                        Foreman::send_callback(*id)
                    )
                );
        }

        self.finish();
    }

    fn wait(&self) {
        io::stdout().flush().expect("Error flushing stdout.");
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("Failed to read from stdin.");
    }

    fn send_callback(miner_id: MinerId) -> impl FnOnce(MiningMessage) {
        // TODO: Implement errors log.
        move |message: MiningMessage| { println!("Error sending {:?} to miner {}", message, miner_id) }
    }

    fn finish(&self) {
        for handle in self.thread_handlers {
            handle.join().unwrap();
        }
    }
}
