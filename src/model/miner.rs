use std::borrow::Borrow;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize};
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use std::collections::HashMap;
use std::{iter, thread};
use std::thread::JoinHandle;
use rand::Rng;

extern crate rand;

use crate::model::communication::MiningMessage;
use crate::model::communication::RoundResults;
use crate::model::communication::MiningMessage::*;
use crate::model::map::Gold;
use crate::utils::logger::Logger;
use crate::utils::utils::CheckedSend;

pub type MinerId = i32;

struct RoundStats {
    gold_dug: Gold,
    results_received: HashMap<MinerId, Gold>,
}

pub struct Miner {
    miner_id: MinerId,
    gold_total: Gold,
    receiving_channel: Receiver<MiningMessage>,
    adjacent_miners: HashMap<MinerId, Sender<MiningMessage>>,
    round: RoundStats,
    logger: Logger,
}

impl Miner {
    pub fn new(id: MinerId, channel_out: Receiver<MiningMessage>, miners: HashMap<MinerId, Sender<MiningMessage>>, logger: Logger) -> Miner {
        Miner {
            miner_id: id,
            gold_total: 0,
            receiving_channel: channel_out,
            adjacent_miners: miners,
            round: RoundStats {
                results_received: HashMap::new(),
                gold_dug: 0,
            },
            logger,
        }
    }

    fn start_mining(&mut self, mut working_flag: Arc<AtomicBool>, mut mining_flag: Arc<AtomicBool>, mut atomic_probability: Arc<AtomicUsize>) {
        while *working_flag.get_mut() {
            self.logger.info(format!("Miner {} started round!", self.miner_id));
            if *mining_flag.get_mut() {
                let probability = *atomic_probability.get_mut()/100;
                self.logger.debug(format!("Miner {} probability: {}", self.miner_id, probability));
                let mut random_generator = rand::thread_rng();
                self.round = RoundStats { results_received: HashMap::new(), gold_dug: 0 };
                self.round.gold_dug = iter::repeat(1)
                    .take(10)
                    .map(|_| random_generator.gen_range(0.0, 1.0) as f64)
                    .filter(|x| x > &probability).count() as Gold;
            }
        }
    }

    fn stop_mining(&mut self) {
        // TODO: Check errors when sending message.
        println!("Miner {} stopped round!", self.miner_id);
        self.adjacent_miners.iter()
            .for_each(|(id, channel)|
                channel.checked_send(
                    ResultsNotification((self.miner_id, self.round.gold_dug)),
                    Miner::send_callback(*id),
                )
            );
    }

    fn save_result(&mut self, (id, gold): RoundResults) { self.round.results_received.insert(id, gold); }

    fn remove_miner(&mut self, id: MinerId) {
        self.adjacent_miners.remove(&id);
    }

    fn receive_gold(&mut self, gold: Gold) {
        self.gold_total += gold;
    }

    fn send_callback(miner_id: MinerId) -> impl FnOnce(MiningMessage) {
        move |message: MiningMessage| { println!("Error sending {:?} to miner {}", message, miner_id) }
    }

    pub fn work(&mut self) {
        let mut working_flag = Arc::new(AtomicBool::new(true));
        let mut working_flag2 = working_flag.clone();

        let mut mining_flag = Arc::new(AtomicBool::new(false));
        let mut mining_flag2 = mining_flag.clone();

        let mut probability = Arc::new(AtomicUsize::new(0));
        let mut probability2 = probability.clone();

        let handler: JoinHandle<()> = thread::spawn(move || {
            self.start_mining(working_flag2, mining_flag2, probability2);
        });

        loop {
            match self.receiving_channel.recv().unwrap() {
                Start(section) => {
                    self.logger.info(format!("Miner {} started round.", self.miner_id));
                    working_flag = Arc::from(true);
                }
                Stop => {
                    self.logger.info(format!("Miner {} started round.", self.miner_id));
                    mining_flag = Arc::from(false);
                    self.stop_mining();
                }
                ResultsNotification(rr) => self.save_result(rr),
                ILeft(id) => self.remove_miner(id),
                TransferGold(gold) => self.receive_gold(gold)
            }
        }

        handler.join().wrap();
    }
}
