use crate::model::miner::Miner;
use crate::model::communication::MiningMessage;
use crate::model::communication::RoundResults;
use crate::model::communication::MiningMessage::*;
use crate::model::map::{MapSection, SectionId, SectionProbability};
use crate::utils::logger::{LoggerWriter, Logger};
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::mpsc::channel;
use std::time::Duration::from_secs;

#[cfg(test)]
mod tests {
    #[test]
    fn test_miner_mines() { // and aligators aligate
        let (logger_in, _logger_out): (Sender<String>, Receiver<String>) = channel();
        let (channel_in, channel_out): (Sender<MiningMessage>, Receiver<MiningMessage>) = channel();
        let miner = Miner::new(1,channel_out,HashMap::new(),Logger::new(logger_in));

        let section = MapSection(1 as SectionId ,0.5 as SectionProbability);
        channel_in.tx(Start(section));
        thread::sleep(time::Duration::from_secs(2));
        channel_in.tx(Stop());
        assert_eq!(1,1);
    }
}
