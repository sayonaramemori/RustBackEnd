use chrono::prelude::*;
use serde::{Deserialize,Serialize};
use sqlx::FromRow;
use std::convert::From;
use super::record::Record;




#[derive(Deserialize,Serialize,Default,Clone,Debug,FromRow)]
pub struct TempRecord<T>
{
    pub val: f64,
    pub id: i64,
    pub time: T,
}

impl<T> TempRecord<T>{
    pub fn new(val:f64,id:i64,time:T) -> TempRecord<T> {
        TempRecord {
            val,
            id,
            time,
        }
    }
}

impl From<TempRecord<DateTime<Utc>>> for TempRecord<NaiveDateTime>{
    fn from(value: TempRecord<DateTime<Utc>>) -> Self {
        TempRecord {
            val: value.val,
            id: value.id,
            time: value.time.naive_local(),
        }
    }
}

impl From<(Record,i64)> for TempRecord<String> {
    fn from(value: (Record,i64)) -> TempRecord<String> {
        let (record,id) = value;
        TempRecord {
            val: record.v.parse::<f64>().unwrap_or(0.0),
            id,
            //only debug string works for frontend
            time: format!("{:?}",record.t),
        }
    }
}

