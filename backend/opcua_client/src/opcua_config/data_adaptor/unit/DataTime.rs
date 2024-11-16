use std::fmt::Display;
use chrono::prelude::*;
use serde::Serialize;
use crate::opcua_config::data_adaptor::interface::collect::{StoreValueTime,RetrieveDataTime};
use crate::utility::time::with_timezone;


#[derive(Clone,Debug,Default,Serialize)]
pub struct DataTime{pub v: String,pub t: DateTime<FixedOffset>}

impl Display for DataTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}|{:?}",self.v,self.t)
    }
}

impl RetrieveDataTime for DataTime {
    fn get_time(&self)->DateTime<Utc> {
        return self.t.naive_local().and_utc();
    }
    fn get_value(&self)-> String{
        return self.v.to_string();
    }
}

impl StoreValueTime for DataTime {
    fn new() -> Self {
        return DataTime::default();
    }
    fn set_time(&mut self,time: DateTime<Utc>) -> &mut Self{
        let china_time = with_timezone(time);
        self.t = china_time;
        self
    }
    fn set_value(&mut self,val: String) -> &mut Self{
        self.v = val;
        self
    }
}