use std::str::FromStr;
use chrono::prelude::*;
use serde::{Deserialize,Serialize};

#[derive(Deserialize,Serialize,Default,Debug)]
pub struct Record{
    pub v: String,
    pub t: DateTime<FixedOffset>
}


/// Handle the raw opcua datas collected.
impl FromStr for Record {
    type Err = serde_json::Error;
    fn from_str(s: &str) -> Result<Self,Self::Err> {
        serde_json::from_str::<Record>(s) 
    }
}