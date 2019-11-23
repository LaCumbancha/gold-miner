use crate::model::map::Gold;
use crate::model::map::MapSection;
use crate::model::miner::MinerId;

pub type  RoundResults = (MinerId,Gold);

pub enum MiningMessage {
    Start(MapSection),
    Stop,
    ResultsNotification(RoundResults),
    ILeft(MinerId),
    TransferGold(Gold)
}

