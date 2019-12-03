use std::{thread, io};
use std::sync::mpsc::{Sender, Receiver, channel};
use std::collections::HashMap;
use std::thread::{JoinHandle, sleep};

extern crate rand;

use rand::Rng;
use rand::prelude::ThreadRng;

use crate::utils::utils::CheckedSend;
use crate::model::miner::Miner;
use crate::model::map::MapSection;
use crate::model::communication::MiningMessage;
use crate::model::communication::MiningMessage::*;
use crate::utils::logger::Logger;
use std::time::Duration;

use crate::model::map::{Gold};
use crate::model::communication::RoundResults;

pub type MinerId = i32;

pub struct Foreman {
    sections: Vec<MapSection>,
    miners_channels: HashMap<MinerId, Sender<MiningMessage>>,
    thread_handlers: Vec<JoinHandle<()>>,
    logger: Logger,
    receiving_channel: Receiver<MiningMessage>,
    channel_in_foreman: Sender<MiningMessage>,
    results_received: HashMap<MinerId, Gold>,
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
            thread_handlers: Vec::new(),
            logger,
            receiving_channel: channel_out_foreman,
            channel_in_foreman: channel_in_foreman,
            results_received: HashMap::new(),
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
            self.results_received.insert(id,0);
        }
        channels_in.insert(0, self.channel_in_foreman.clone());

        for id in 1..=miners {
            // Preparing channels for each miner.
            let miner_receiving_channel = channels_out.remove(&id).unwrap();
            let mut miner_adjacent_channels = channels_in.clone();
            miner_adjacent_channels.remove(&id);
            let miner_logger = self.logger.clone();

            self.logger.info(format!("Creating miner {}", id.clone()));
            let handler: JoinHandle<()> = thread::spawn(move || {
                let mut miner: Miner = Miner::new(id, miner_receiving_channel, miner_adjacent_channels, miner_logger);
                miner.work();
            });

            self.thread_handlers.push(handler);
        }
    }

    pub fn start_mining(&mut self) {
        println!("FOREMAN: Ok, it's showtime. Let's get this shit done.");

        for section in self.sections.clone() {
            if self.miners_channels.len() == 1 { break; }
            println!();
            println!("FOREMAN: Yo' filthy rats! Go find me some gold in Section {}! (Press [ENTER] to make miners start digging)", section.0);
            self.wait();
            self.logger.info(format!("In Section {} there is {} probability of extracting gold", section.0, 1.0 - section.1));

            self.miners_channels.iter().for_each(|(id, channel)|
                channel.checked_send(
                    Start(section.clone()),
                    Foreman::send_callback(id.clone(), self.logger.clone()),
                )
            );

            println!("Press [ENTER] to make miners stop digging.");
            self.wait();

            self.miners_channels.iter().for_each(|(id, channel)|
                channel.checked_send(
                    Stop,
                    Foreman::send_callback(id.clone(), self.logger.clone()),
                )
            );

            let mut miners_ready = 0;
            while miners_ready!=self.miners_channels.len() {
                match self.receiving_channel.recv().unwrap() {
                    Ready => {miners_ready+=1;},
                    ResultsNotification(results) => self.save_result(results),
                    ILeft(id) => self.remove_miner(id),
                    _=>{}
                }
            }
        }
        self.finish();
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

    fn finish(&mut self) {
        self.miners_channels.iter().for_each(|(id, channel)|
            channel.checked_send(
                ByeBye,
                Foreman::send_callback(id.clone(), self.logger.clone()),
            )
        );

        // TODO: Join handlers
        sleep(Duration::from_secs(1));
        // for handler in self.thread_handlers.iter_mut() {
        //     handler.join().unwrap();
        // }
        println!();
        self.results_received.iter()
        .for_each(|(id, gold)|{
            println!("Miner number {} extracted {} gold", id, gold);
        });
    }

    fn save_result(&mut self, (id, gold): RoundResults) { 
        //TODO: que sume las pepitas de cada minero*************************************
/*
        let gold_old = self.results_received.get(&id);
        self.results_received.insert(id, &gold_old+gold);*/

        //self.results_received.insert(id, gold);
        //TODO: check who is the miner with less gold
        if let Some(x) = self.results_received.get_mut(&id) {
            *x = *x+gold;
        }
        //assert_eq!(map[&1], "b");
    }

    fn remove_miner(&mut self, id: MinerId) {
        self.miners_channels.remove(&id);
    }
}
