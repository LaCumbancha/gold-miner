use std::sync::Arc;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use std::sync::Mutex;
use std::collections::HashMap;
use std::thread;
use rand::{thread_rng, Rng};
use crate::model::communication::MiningMessage;
use crate::model::communication::RoundResults;
use crate::model::communication::MiningMessage::*;
use crate::model::map::Gold;
use crate::model::map::SectionProbability;
use crate::utils::logger::Logger;
use crate::utils::utils::CheckedSend;
pub type MinerId = i32;

struct RoundStats {
    gold_dug: Arc<Mutex<Gold>>,
    results_received: HashMap<MinerId, Gold>,
}

pub struct Miner {
    miner_id: MinerId,
    gold_total: Gold,
    receiving_channel: Receiver<MiningMessage>,
    adjacent_miners: HashMap<MinerId, Sender<MiningMessage>>,
    round: RoundStats,
    keep_mining: Arc<Mutex<bool>>,
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
                gold_dug: Arc::new(Mutex::new(0 as Gold)),
            },
            keep_mining: Arc::new(Mutex::new(false)),
            logger,
        }
    }
    fn mine(keep_mining: Arc<Mutex<bool>>, gold_dug: Arc<Mutex<Gold>>, prob: SectionProbability) {
        *gold_dug.lock().unwrap() = 0;
        let mut rng = thread_rng();
        while *keep_mining.lock().unwrap() {
            *gold_dug.lock().unwrap() += rng.gen_bool(prob) as i32;
        }
    }

    fn start_mining(&mut self, prob: SectionProbability) {
        let mut keep_mining = self.keep_mining.lock().unwrap();
        *keep_mining = true;

        self.round.results_received = HashMap::new();
        *self.round.gold_dug.lock().unwrap() = 0;

        let keep_mining = Arc::clone(&self.keep_mining);
        let gold_dug = Arc::clone(&self.round.gold_dug);
        let prob_clone = prob.clone();
        Some(thread::spawn(move || { Miner::mine(keep_mining, gold_dug, prob_clone) }));

        self.logger.debug(format!("Miner {} started round!", self.miner_id));
    }

    fn stop_mining(&mut self) {
        let mut keep_mining = self.keep_mining.lock().unwrap();
        *keep_mining = false;

        let gold_dug = self.round.gold_dug.lock().unwrap();
        self.logger.info(format!("Miner {} stopped round! He got {} pieces of gold dug.", self.miner_id, *gold_dug));
        // self.observers.iter()
        //     .for_each(|channel|
        //               channel.send(
        //                   ResultsNotification((self.miner_id, *gold_dug))
        //               ));
        self.adjacent_miners.iter()
            .for_each(|(id, channel)|
                channel.checked_send(
                    ResultsNotification((self.miner_id, *gold_dug)),
                    Miner::send_callback(*id, self.logger.clone()),
                )
            );


        // TODO: Uncomment when communication with foreman is established.
        println!("MINER #{}: I've found {} pieces of gold!", self.miner_id, gold_dug);
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

    fn send_callback(miner_id: MinerId, logger: Logger) -> impl FnOnce(MiningMessage) {
        move |message: MiningMessage| {
            logger.error(format!("Error sending {:?} to miner {}", message, miner_id))
        }
    }

    pub fn work(&mut self) {
        loop {
            match self.receiving_channel.recv().unwrap() {
                Start(section) => self.start_mining(section.1),
                Stop => self.stop_mining(),
                ResultsNotification(results) => self.save_result(results),
                ILeft(id) => self.remove_miner(id),
                TransferGold(gold) => self.receive_gold(gold),
                ByeBye => {
                    self.logger.info(format!("Miner {} finished working!", self.miner_id));
                    break;
                }
            }
        }
    }
}

