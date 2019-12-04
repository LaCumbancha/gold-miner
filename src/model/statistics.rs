use crate::model::map::Gold;
use crate::model::miner::MinerId;
use std::collections::HashMap;
extern crate csv;
use std::error::Error;
use csv::ReaderBuilder;
use csv::Writer;
use csv::Reader;
extern crate serde;
use serde::Deserialize;
use serde::Serialize;


#[derive(Debug,Deserialize,Serialize)]
pub struct RoundResult {
    id:MinerId,
    gold:Gold,
}
pub type StatFile = String;
pub fn log_stat(stat_file:StatFile,result:RoundResult) -> Result<(),Box<Error>>{
    let mut wtr = Writer::from_path(stat_file)?;
    wtr.serialize(result)?;
    wtr.flush()?;
    Ok(())
}

pub fn net_gold(stat_file:StatFile)-> Result<HashMap<MinerId,Gold>,Box<Error>>{
    let mut rdr = Reader::from_path(stat_file)?;
    let mut total:HashMap<MinerId,Gold> = HashMap::new();
    for result in rdr.deserialize(){
        let record:RoundResult = result?;
    }
    Ok(total)
}

pub fn gold_dug(stat_file:StatFile)-> Result<HashMap<MinerId,Gold>,Box<Error>>{
    let mut rdr = Reader::from_path(stat_file)?;
    let mut total:HashMap<MinerId,Gold> = HashMap::new();
    for r in rdr.deserialize(){
        let result:RoundResult = r?;
        total.entry(result.id).and_modify(|g| *g+=result.gold).or_insert(result.gold);
    }
    Ok(total)
}
