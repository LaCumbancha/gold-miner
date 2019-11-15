use crate::model::map::Gold;
use crate::model::map::MapSegment;
pub enum MiningMessage{
    Start(MapSegment),
    Stop,
    NotifyResults,
    ILeft,
    TransferGold(Gold)
}
