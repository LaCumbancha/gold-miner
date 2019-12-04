extern crate termion;

use termion::style;

use std::{thread, io};
use std::sync::mpsc::{Sender, Receiver, channel};
use std::collections::{HashMap, BTreeMap};
use std::thread::JoinHandle;

extern crate rand;

use rand::Rng;
use rand::prelude::ThreadRng;

use crate::utils::extension::CheckedSend;
use crate::model::miner::Miner;
use crate::model::map::MapSection;
use crate::model::communication::MiningMessage;
use crate::model::communication::MiningMessage::*;
use crate::utils::logger::Logger;

use crate::model::map::Gold;
use crate::model::communication::RoundResults;

pub type MinerId = i32;

pub struct Foreman {
    sections: Vec<MapSection>,
    miners_channels: HashMap<MinerId, Sender<MiningMessage>>,
    logger: Logger,
    receiving_channel: Receiver<MiningMessage>,
    sending_channel: Sender<MiningMessage>,
    results_received: BTreeMap<MinerId, Gold>,
}

impl Foreman {
    pub fn new(sections: i32, logger: Logger) -> Foreman {
        println!("FOREMAN: Welcome to the Gold Camp! I'm the foreman, the man in charge. Hope we finally get some gold.");
        println!("FOREMAN: Today we'll be exploring this {} zones.", sections);

        // Generating sections randomly.
        let mut random_generator: ThreadRng = rand::thread_rng();
        let mut region_sections: Vec<MapSection> = Vec::new();
        for section_id in 1..=sections {
            region_sections.push((section_id, random_generator.gen_range(0.0, 1.0)));
        }

        // Creating foreman channel.
        let (channel_in_foreman, channel_out_foreman): (Sender<MiningMessage>, Receiver<MiningMessage>) = channel();

        Foreman {
            sections: region_sections,
            miners_channels: HashMap::new(),
            logger,
            receiving_channel: channel_out_foreman,
            sending_channel: channel_in_foreman,
            results_received: BTreeMap::new(),
        }
    }

    pub fn work(&mut self, miners: i32) {
        println!("FOREMAN: But first, we need some cheap manpower. We'll go to the town and get the first {} morons that show up.", miners);

        // Creating channels for every miner.
        let mut channels_in: HashMap<MinerId, Sender<MiningMessage>> = HashMap::new();
        let mut channels_out: HashMap<MinerId, Receiver<MiningMessage>> = HashMap::new();
        for id in 1..=miners {
            let (channel_in, channel_out): (Sender<MiningMessage>, Receiver<MiningMessage>) = channel();
            self.miners_channels.insert(id, channel_in.clone());
            channels_in.insert(id, channel_in);
            channels_out.insert(id, channel_out);
            self.results_received.insert(id, 0);
        }
        channels_in.insert(0, self.sending_channel.clone());

        let handlers: Vec<JoinHandle<()>> = self.hire_miners(miners, channels_in, channels_out);
        self.start_mining();
        self.finish(handlers);
    }

    fn hire_miners(&mut self, miners: i32, channels_in: HashMap<MinerId, Sender<MiningMessage>>, mut channels_out: HashMap<MinerId, Receiver<MiningMessage>>) -> Vec<JoinHandle<()>> {
        let mut handlers = Vec::new();
        for id in 1..=miners {
            // Preparing channels for each miner.
            let miner_receiving_channel = channels_out.remove(&id).unwrap();
            let mut miner_adjacent_channels = channels_in.clone();
            miner_adjacent_channels.remove(&id);
            let miner_logger = self.logger.clone();

            self.logger.info(format!("Creating miner {}", id.clone()));
            let handler: JoinHandle<_> = thread::spawn(move || {
                let mut miner: Miner = Miner::new(id, miner_receiving_channel, miner_adjacent_channels, miner_logger);
                miner.work();
            });

            handlers.push(handler);
        }
        return handlers;
    }

    pub fn start_mining(&mut self) {
        println!("FOREMAN: Ok, it's showtime. Let's get this shit done.");

        for section in self.sections.clone() {
            if self.miners_channels.len() == 1 { break; }
            println!();
            println!("FOREMAN: Yo' filthy rats! Go find me some gold in Section {}!", section.0);
            println!("(Press [ENTER] to make miners {}start{} digging and [ENTER] again to make them {}stop{})", style::Bold, style::Reset, style::Bold, style::Reset);
            self.logger.info(format!("In Section {} there is {} probability of extracting gold.", section.0, 1.0 - section.1));

            self.wait();
            self.miners_channels.iter().for_each(|(id, channel)| {
                self.logger.debug(format!("Foreman sending a 'Start' message to miner {}.", id));
                channel.checked_send(
                    Start(section.clone()),
                    Foreman::send_callback(id.clone(), self.logger.clone()),
                )
            });

            self.wait();
            self.miners_channels.iter().for_each(|(id, channel)| {
                self.logger.debug(format!("Foreman sending a 'Stop' message to miner {}.", id));
                channel.checked_send(
                    Stop,
                    Foreman::send_callback(id.clone(), self.logger.clone()),
                )
            });

            let mut miners_ready = 0;
            while miners_ready != self.miners_channels.len() {
                match self.receiving_channel.recv().unwrap() {
                    Ready(id) => {
                        self.logger.debug(format!("Foreman received an 'I'm Ready' message from miner {}.", id));
                        miners_ready += 1;
                    }
                    ResultsNotification((id, gold)) => {
                        self.logger.debug(format!("Foreman received {} pieces of gold from miner {}.", gold, id));
                        self.save_result((id, gold));
                    }
                    ILeft(id) => {
                        self.remove_miner(id);
                        self.logger.info(format!("Foreman received an 'I Left' message from miner {}.", id));
                    }
                    _ => {}
                }
            }
        }
    }

    fn wait(&self) {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("Failed to read from stdin.");
    }

    fn send_callback(miner_id: MinerId, logger: Logger) -> impl FnOnce(MiningMessage) {
        move |message: MiningMessage| {
            logger.error(format!("Error sending {:?} to miner {}", message, miner_id))
        }
    }

    fn finish(&mut self, handlers: Vec<JoinHandle<()>>) {
        self.miners_channels.iter().for_each(|(id, channel)| {
            self.logger.debug(format!("Foreman sending a 'ByeBye' message to miner {}.", id));
            channel.checked_send(
                ByeBye,
                Foreman::send_callback(id.clone(), self.logger.clone()),
            );
        });

        for handler in handlers {
            handler.join().unwrap();
        }

        println!();
        println!("FOREMAN: Arrgg, what a lovely day. Let's see what you got me!");
        self.results_received.iter().for_each(|(id, gold)| {
            println!("MINER #{}: Extracted {} pieces of gold.", id, gold);
        });
    }

    fn save_result(&mut self, (id, gold): RoundResults) {
        if let Some(x) = self.results_received.get_mut(&id) {
            *x = *x + gold;
        }
    }

    fn remove_miner(&mut self, id: MinerId) {
        self.miners_channels.remove(&id);
    }
}
