use std::sync::mpsc::Sender;
use std::collections::HashMap;
use std::sync::mpsc::Receiver;
use crate::model::map::Gold;
use crate::model::map::MapSegment;
use crate::model::communication::MiningMessage;
use crate::model::communication::MiningMessage::*;
use std::sync::mpsc::channel;
pub type MinerId = i32;

struct RoundStats{
    results_recvd:i32,
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
              round:RoundStats{results_recvd:0,
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

    pub fn main(&self){
        loop{
            match self.rx.recv().unwrap() {
                Start(mseg) => panic!("Not implemented"),
                Stop => panic!("Not implemented"),
                NotifyResults => panic!("Not implemented"),
                ILeft => panic!("Not implemented"),
                TransferGold(g) => panic!("Not implemented"),
                _ =>  panic!("Not understood")
            }
        }
    }
}
