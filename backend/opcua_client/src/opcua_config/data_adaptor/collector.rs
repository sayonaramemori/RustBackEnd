use tokio::time::sleep;
use tokio::sync::broadcast::{self,Sender,Receiver};
use mysql::*;
use std::sync::Arc;
use std::usize;
use crate::debug_println;
use crate::opcua_config::opcua_session_wrapper::OpcuaSession;
use super::interface::collect::StoreValueTime;

pub struct DataCollector<T: Send + Sync> {
    target: String,
    sender: Arc<Sender<T>>,
    //default 1
    collect_interval: usize,
    endpoint_url: String,
}

impl<T> DataCollector<T> 
where T: 'static + Clone + Send + Sync + StoreValueTime
{
    pub fn new(target:&str,pipe_size: usize, endpoint_url: &str) -> DataCollector<T> {
        let (sender, _rx) = broadcast::channel::<T>(pipe_size);
        let res = DataCollector { 
            target: target.to_string(), 
            sender: Arc::new(sender),
            collect_interval: 1,
            endpoint_url: endpoint_url.to_string(),
        };
        return res;
    }

    //Read node only once
    pub async fn execute(&self) -> Result<(),String>{
        let session= OpcuaSession::new_arc(self.endpoint_url.clone()).await;
        match OpcuaSession::async_read(session,self.target.as_ref()).await {
            Ok(res) => { let _ = self.sender.send(res); },
            Err(e) => { return Err(format!("Read node failed for {e}")); },
        }
        Ok(())
    }

    //Read node within loop ,called after necessary subscription
    pub async fn execute_loop(collector: DataCollector<T>) -> Result<(),String>{
        collector.start_with_one_target().await;
        Ok(())
    }

    async fn start_with_one_target(&self){
        let session= OpcuaSession::new_arc(self.endpoint_url.clone()).await;
        loop {
            let session_clone = session.clone();
            match OpcuaSession::async_read(session_clone,self.target.as_ref()).await {
                Ok(res) => {
                    let _ = self.sender.send(res);
                },
                Err(_e) => { debug_println!("Reading node failed"); },
            };
            sleep(std::time::Duration::from_secs(self.collect_interval as u64)).await;
        }
    }

    pub fn subscribe(&self) -> Receiver<T>{
        self.sender.subscribe()
    }

    pub fn set_interval(&mut self, interval:usize){
        self.collect_interval = interval;
    }
}

