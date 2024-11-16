use serde::{Deserialize,Serialize};
use crate::models::entities::temp_record::TempRecord;

#[derive(Deserialize,Serialize,Default,Debug)]
pub struct HistoryData<T>
{
    pub average: f64,
    pub total_time: f64,
    pub records: Vec<TempRecord<T>>,
}