use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use std::collections::HashMap;
use std::iter;

extern crate rand;

use rand::Rng;

use crate::utils::utils::CheckedSend;
use crate::model::map::{Gold, SectionProbability};
use crate::model::communication::MiningMessage;
use crate::model::communication::RoundResults;
use crate::model::communication::MiningMessage::*;
use crate::utils::logger::Logger;

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
            logger
        }
    }

    fn start_round(&mut self, message: SectionProbability) {
        println!("Miner {} started round!", self.miner_id);
        let mut random_generator = rand::thread_rng();
        self.round = RoundStats { results_received: HashMap::new(), gold_dug: 0 };
        self.round.gold_dug = iter::repeat(1)
            .take(10)
            .map(|_| random_generator.gen_range(0.0, 1.0) as f64)
            .filter(|x| x > &message as &f64).count() as Gold;
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
        loop {
            match self.receiving_channel.recv().unwrap() {
                Start(section) => self.start_round(section.1),
                Stop => self.stop_mining(),
                ResultsNotification(rr) => self.save_result(rr),
                ILeft(id) => self.remove_miner(id),
                TransferGold(g) => self.receive_gold(g)
            }
        }
    }
}
