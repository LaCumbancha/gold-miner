use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::channel;
use std::collections::HashMap;
use std::iter;
extern crate rand;
use rand::Rng;
use rand::prelude::*;
use crate::model::map::Gold;
use crate::model::map::MapSegment;
use crate::model::communication::MiningMessage;
use crate::model::communication::RoundResults;
use crate::model::communication::MiningMessage::*;

pub type MinerId = i32;

struct RoundStats{
    results_recvd:HashMap<MinerId,Gold>,
    gold_dug:Gold
}
pub struct Miner {
    tx:Sender<MiningMessage>,
    rx:Receiver<MiningMessage>,
    adjacents:HashMap<MinerId,Sender<MiningMessage>>,
    id:MinerId,
    gold_total:Gold,
    round: RoundStats
}

impl Miner{
    pub fn new(id:MinerId) -> Miner{
        let (tx,rx):(Sender<MiningMessage>,Receiver<MiningMessage>) = channel();
        Miner{tx,rx,
              adjacents:HashMap::new(),
              id,
              gold_total:0,
              round:RoundStats{results_recvd: HashMap::new(),
                               gold_dug:0}
        }
    }
    pub fn meet(&mut self,miner:Miner){
        let (id,tx) = miner.contact();
        self.adjacents.insert(id,tx);
    }

    pub fn contact(&self)->(MinerId,Sender<MiningMessage>){
        (self.id.clone(),self.tx.clone())
    }

    fn start_round(&mut self,mseg:MapSegment){
        let mut rng = rand::thread_rng();
        self.round = RoundStats{results_recvd: HashMap::new(), gold_dug:0};
        self.round.gold_dug=iter::repeat(1)
            .take(10)
            .map(|_| rng.gen_range(0.0,1.0) as f64 )
            .filter(|x| x  > &mseg as &f64).count() as Gold;
    }
    fn stop_mining(&mut self){
        self.adjacents.values()
            .map(|v| v.send(ResultsNotification((self.id,self.round.gold_dug))));
    }
    fn save_result(&mut self,(id,gold):RoundResults){
        self.round.results_recvd.insert(id,gold);
    }

    fn remove_miner(&mut self,id:MinerId){
        self.adjacents.remove(&id);
    }
    fn receive_gold(&mut self,gold:Gold){
        self.gold_total += gold;
    }
    pub fn main(&mut self){
        loop{
            match self.rx.recv().unwrap() {
                Start(mseg) => self.start_round(mseg),
                Stop => self.stop_mining(),
                ResultsNotification(rr) =>self.save_result(rr),
                ILeft(id) => self.remove_miner(id),
                TransferGold(g) => self.receive_gold(g),
                _ =>  panic!("Not understood")
            }
        }
    }
}
