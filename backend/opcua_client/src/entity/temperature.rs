use std::convert::From;
use chrono::prelude::NaiveDateTime;

use crate::opcua_config::data_adaptor::interface::collect::RetrieveDataTime;

#[derive(Default,Debug)]
pub struct Temperature {
    pub val: f64,
    pub time: NaiveDateTime,
}

impl<T> From<T> for Temperature 
where T: RetrieveDataTime 
{
    fn from(value: T) -> Self {
        let val = value.get_value().parse::<f64>().unwrap_or(0.1234567);
        let time = value.get_time().naive_local();
        Temperature {
            val,
            time,
        }
    }
}
