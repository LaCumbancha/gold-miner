use crate::model::map::Gold;
use crate::model::map::MapSegment;
use crate::model::miner::MinerId;
pub type  RoundResults = (MinerId,Gold);
pub enum MiningMessage{
    Start(MapSegment),
    Stop,
    ResultsNotification(RoundResults),
    ILeft(MinerId),
    TransferGold(Gold)
}

