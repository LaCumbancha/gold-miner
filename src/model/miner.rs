use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::channel;
use std::collections::HashMap;
use std::iter;

extern crate rand;

use rand::Rng;
use crate::model::map::{Gold, SectionProbability};
use crate::model::communication::MiningMessage;
use crate::model::communication::RoundResults;
use crate::model::communication::MiningMessage::*;

pub type MinerId = i32;

struct RoundStats {
    gold_dug: Gold,
    results_received: HashMap<MinerId, Gold>,
}

pub struct Miner {
    miner_id: MinerId,
    gold_total: Gold,
    sending_channel: Sender<MiningMessage>,
    receiving_channel: Receiver<MiningMessage>,
    adjacent_miners: HashMap<MinerId, Sender<MiningMessage>>,
    round: RoundStats,
}

impl Miner {
    pub fn new(id: MinerId) -> Miner {
        let (channel_in, channel_out): (Sender<MiningMessage>, Receiver<MiningMessage>) = channel();

        Miner {
            miner_id: id,
            gold_total: 0,
            sending_channel: channel_in,
            receiving_channel: channel_out,
            adjacent_miners: HashMap::new(),
            round: RoundStats {
                results_received: HashMap::new(),
                gold_dug: 0,
            },
        }
    }

    pub fn meet(&mut self, miner: Miner) {
        let (miner_id, sending_channel) = miner.contact();
        self.adjacent_miners.insert(miner_id, sending_channel);
    }

    pub fn contact(&self) -> (MinerId, Sender<MiningMessage>) {
        (self.miner_id.clone(), self.sending_channel.clone())
    }

    fn start_round(&mut self, message: SectionProbability) {
        let mut rng = rand::thread_rng();
        self.round = RoundStats { results_received: HashMap::new(), gold_dug: 0 };
        self.round.gold_dug = iter::repeat(1)
            .take(10)
            .map(|_| rng.gen_range(0.0, 1.0) as f64)
            .filter(|x| x > &message as &f64).count() as Gold;
    }

    fn stop_mining(&mut self) {
        // TODO: Check errors when sending message.
        self.adjacent_miners.values()
            .map(|miner| miner.send(ResultsNotification((self.miner_id, self.round.gold_dug))));
    }

    fn save_result(&mut self, (id, gold): RoundResults) {
        self.round.results_received.insert(id, gold);
    }

    fn remove_miner(&mut self, id: MinerId) {
        self.adjacent_miners.remove(&id);
    }

    fn receive_gold(&mut self, gold: Gold) {
        self.gold_total += gold;
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
