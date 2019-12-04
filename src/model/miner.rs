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

use std::cmp;

pub type MinerId = i32;

struct RoundStats {
    gold_dug: Arc<Mutex<Gold>>,
    results_received: HashMap<MinerId, Gold>,
}

pub struct Miner {
    miner_id: MinerId,
    gold_total: Gold,
    gold_dug: Gold,
    receiving_channel: Receiver<MiningMessage>,
    adjacent_miners: HashMap<MinerId, Sender<MiningMessage>>,
    round: RoundStats,
    keep_mining: Arc<Mutex<bool>>,
    logger: Logger,
    wait: i32,
}

impl Miner {
    pub fn new(id: MinerId, channel_out: Receiver<MiningMessage>, miners: HashMap<MinerId, Sender<MiningMessage>>, logger: Logger) -> Miner {
        Miner {
            miner_id: id,
            gold_total: 0,
            gold_dug: 0,
            receiving_channel: channel_out,
            adjacent_miners: miners,
            round: RoundStats {
                results_received: HashMap::new(),
                gold_dug: Arc::new(Mutex::new(0 as Gold)),
            },
            keep_mining: Arc::new(Mutex::new(false)),
            logger,
            wait: 0,
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
        self.adjacent_miners.iter()
            .for_each(|(id, channel)|
                channel.checked_send(
                    ResultsNotification((self.miner_id, *gold_dug)),
                    Miner::send_callback(*id, self.logger.clone()),
                )
            );

        self.gold_total += *gold_dug;
        self.gold_dug += *gold_dug;
        println!("MINER #{}: I've found {} pieces of gold!", self.miner_id, gold_dug);
    }

    fn save_result(&mut self, (id, gold): RoundResults) -> bool{
        self.round.results_received.insert(id, gold);
        if self.round.results_received.len() == self.adjacent_miners.len() - 1 { //All miners stoped and sended their results
            let (minor, major) = self.get_min_max_round_results();
            if minor.len() == 1 {
                self.logger.debug(format!("Miner {} detected a round loser.", self.miner_id));
                if minor[0].0 == self.miner_id {
                    self.logger.info(format!("Miner {} detected himself as the round loser.", self.miner_id));
                    self.adjacent_miners.iter()
                        .for_each(|(id, channel)| {
                            if id == &major[0].0 {
                                self.logger.info(format!("Miner {} send his {} pieces of gold to miner {}.", self.miner_id, self.gold_total, id));
                                channel.checked_send(
                                    TransferGold(self.gold_total),
                                    Miner::send_callback(*id, self.logger.clone()),
                                )
                            };
                            self.logger.info(format!("Miner {} send an 'I Left' message to miner {}.", self.miner_id, id));
                            channel.checked_send(
                                ILeft(self.miner_id),
                                Miner::send_callback(*id, self.logger.clone()),
                            )
                        });
                    return true;
                }
                self.wait += 1;
                if major[0].0 == self.miner_id {//this miner is the winner
                    self.wait += 1;
                    return false; //wait gold dugs
                }
            }

            //There is not loser or this miner is not the winner
            //says "I'm ready" to the foreman
            self.ready();
            
        }
        return false;
    }

    fn ready(&mut self) {
        if self.wait != 0 {
            return;
        }
        self.adjacent_miners.iter()
            .for_each(|(id, channel)| {
                if id == &0 {
                    self.logger.info(format!("Miner {} send an 'I'm Ready' message to foreman.", self.miner_id));
                    channel.checked_send(
                        Ready(self.miner_id),
                        Miner::send_callback(*id, self.logger.clone()),
                    )
                };
            });

        /*let id_foreman = 0;
        let foreman_channel = self.adjacent_miners.get(&id_foreman);
        foreman_channel.checked_send(
            Ready,
            Miner::send_callback(id_foreman, self.logger.clone())
        );*/
    }

    fn remove_miner(&mut self, id: MinerId) {
        self.adjacent_miners.remove(&id);
        self.wait -= 1;
        self.ready();
    }

    fn receive_gold(&mut self, gold: Gold) {
        self.gold_total += gold;
        //says "I'm ready" to the foreman
        self.wait -= 1;
        self.ready();
    }

    fn send_callback(miner_id: MinerId, logger: Logger) -> impl FnOnce(MiningMessage) {
        move |message: MiningMessage| {
            logger.error(format!("Error sending {:?} to miner {}", message, miner_id))
        }
    }

    pub fn work(&mut self) {
        loop {
            match self.receiving_channel.recv().unwrap() {
                Start(section) => {
                    self.logger.debug(format!("Miner {} received a 'Start' message.", self.miner_id));
                    self.start_mining(section.1)
                },
                Stop => {
                    self.logger.debug(format!("Miner {} received a 'Stop' message.", self.miner_id));
                    self.stop_mining()
                },
                ResultsNotification(results) => {
                    self.logger.debug(format!("Miner {} was informed that miner {} dug {} pieces of gold.", self.miner_id, results.0, results.1));
                    if self.save_result(results) == true{
                        break;
                    }
                },
                ILeft(id) => {
                    self.logger.debug(format!("Miner {} received an 'I Left' message from miner {}.", self.miner_id, id));
                    self.remove_miner(id)
                },
                TransferGold(gold) => {
                    self.logger.debug(format!("Miner {} received {} pieces of gold from the round loser.", self.miner_id, gold));
                    self.receive_gold(gold)
                },
                ByeBye => {
                    self.logger.info(format!("Miner {} finished working!", self.miner_id));
                    println!("MINER #{} dug: {} and finished with {}", self.miner_id, self.gold_dug, self.gold_total);
                    //sleep(Duration::from_secs(10));
                    break;
                }
                _ => {}
            }
        }
    }

    fn get_min_max_round_results(&mut self) -> (Vec<(MinerId, Gold)>, Vec<(MinerId, Gold)>) {
        let mut minor: Vec<(MinerId, Gold)> = Vec::new();
        minor.push((0, i32::max_value() as Gold));
        let mut major: Vec<(MinerId, Gold)> = Vec::new();
        major.push((0, i32::min_value() as Gold));

        self.round.results_received.insert(self.miner_id, self.round.gold_dug.lock().unwrap().clone());
        self.round.results_received.iter()
            .for_each(|(id, gold)| {
                if minor[0].1 > *gold {
                    minor.clear();
                    minor.push((*id,*gold));
                   // println!("el menor oro es del id {}", *id);
                }else if minor[0].1 == *gold{
                    minor.push((*id,*gold))
                };

                if major[0].1 <= *gold{//there is only winner
                    let mut min_id = *id;
                    if major[0].1 == *id{
                        min_id = cmp::min(major[0].0, *id);
                    }
                    major.clear();
                    major.push((min_id, *gold));
                }
            }
            );
        return (minor, major);
    }
}

