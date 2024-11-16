use chrono::prelude::*;
pub trait StoreValueTime : Default{
    fn new() -> Self { 
        Self::default() 
    }
    fn set_value(&mut self,val: String)->&mut Self {
        self
    }
    fn set_time(&mut self,time: DateTime<Utc>)->&mut Self{
        self
    }
}

pub trait RetrieveDataTime {
    fn get_time(&self)->DateTime<Utc> {
        DateTime::default()
    }
    fn get_value(&self)-> String{
        String::new()
    }
}