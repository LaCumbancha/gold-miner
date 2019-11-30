use std::sync::Arc;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use std::sync::Mutex;
use std::collections::HashMap;
use std::{thread};
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
    fn mine(keep_mining: Arc<Mutex<bool>>,gold_dug:Arc<Mutex<Gold>>, prob: SectionProbability){
	      *gold_dug.lock().unwrap() = 0;
        let mut rng = thread_rng();
	      while *keep_mining.lock().unwrap() {
	          *gold_dug.lock().unwrap()+=rng.gen_bool(prob) as i32;
	      }
    }

    fn start_mining(&mut self, prob: SectionProbability) {
	      let mut keep_mining = self.keep_mining.lock().unwrap();
	      *keep_mining = true;

	      let keep_mining = Arc::clone(&self.keep_mining);
	      let gold_dug = Arc::clone(&self.round.gold_dug);
	      let prob_clone = prob.clone();
	      Some(thread::spawn(move || {Miner::mine(keep_mining,gold_dug, prob_clone)}));

	      println!("Miner {} started round!", self.miner_id);
        self.round = RoundStats { results_received: HashMap::new(), gold_dug: Arc::new(Mutex::new(0 as Gold)) };

    }

    fn stop_mining(&mut self) {
        // TODO: Check errors when sending message.
        println!("Miner {} stopped round!", self.miner_id);
	      let mut keep_mining = self.keep_mining.lock().unwrap();
	      *keep_mining = false;
	      let gold_dug = self.round.gold_dug.lock().unwrap();
	      self.adjacent_miners.iter()
            .for_each(|(id, channel)|
                      channel.checked_send(
			                    ResultsNotification((self.miner_id, *gold_dug)),
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
                Start(section) => self.start_mining(section.1),
                Stop => self.stop_mining(),
                ResultsNotification(rr) => self.save_result(rr),
                ILeft(id) => self.remove_miner(id),
                TransferGold(g) => self.receive_gold(g)
            }
        }

    }
}

