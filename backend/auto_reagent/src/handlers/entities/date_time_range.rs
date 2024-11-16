use serde::{Deserialize,Serialize};
use chrono::NaiveDateTime;

#[derive(Deserialize,Serialize,Debug)]
pub struct StringDateTimeRng {
    pub start: String, // or chrono::NaiveDateTime if you want to parse dates
    pub end: String,   // or chrono::NaiveDateTime
}


