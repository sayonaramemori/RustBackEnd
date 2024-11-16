use std::sync::Arc;
use crate::opcua_config::data_adaptor::interface::transfer::InstructionInfo;
use tokio::task;
use opcua::{client::prelude::{Client, Session,*}, sync::*};
use crate::debug_println;
use crate::utility::time::*;
use crate::opcua_config::node_config::NodeConfig::get_node_config;
use super::data_adaptor::unit::DataTime::DataTime;
use super::data_adaptor::interface::collect::StoreValueTime;


pub struct OpcuaSession {
    session: Arc<std::sync::RwLock<Option<Arc<RwLock<Session>>>>>,
    url: String,
}

impl OpcuaSession {
    fn get_client()-> Client {
        ClientBuilder::new()
            .application_name("My First Client")
            .application_uri("urn:MyFirstClient")
            .create_sample_keypair(true)
            .trust_server_certs(true)
            .session_retry_limit(9)
            // .session_timeout(999999999)
            .client().unwrap()
    }

    fn gain_new_session(&self){
        let mut session = self.session.write().unwrap();
        *session = Some(Self::get_client()
            .connect_to_endpoint(self.url.as_ref(), IdentityToken::Anonymous)
            .expect("connect failed"));
    }

    fn read_batch(&self, node_id: Vec<NodeId>) -> Result<Vec<DataTime>,StatusCode>{
        let guard = self.session.read().unwrap();
        let session = guard.as_ref().unwrap().read();
        let temp :Vec<ReadValueId>= node_id.into_iter().map(|v|{ReadValueId::from(v)}).collect();
        match session.read(&temp, TimestampsToReturn::Both, 0.0){
            Ok(value) => {
                let mut res:Vec<DataTime> = vec![];
                for i in value{
                    let time = i.server_timestamp.unwrap().as_chrono();
                    let time =with_timezone(time);
                    match i.value {
                        Some(data) => res.push(DataTime{v:data.to_string(),t:time}),
                        _ => return Err(StatusCode::BadNotReadable),
                    }
                }
                Ok(res)
            },
            Err(err) => {
                debug_println!("Read batch failed for {:?}",err);
                drop(session);
                drop(guard);
                self.gain_new_session();
                return Err(err);
            },
        }
    }

    fn read_single<T>(&self, node_id: &NodeId) -> Result<T,StatusCode>
    where T: StoreValueTime
    {
        let guard = self.session.read().unwrap();
        let session = guard.as_ref().unwrap().read();
        let temp :Vec<ReadValueId>= vec![node_id.into()];
        match session.read(&temp, TimestampsToReturn::Both, 0.0){
            Ok(res) => {
                for i in res {
                    let time = i.server_timestamp.unwrap().as_chrono();
                    if let Some(val) = i.value {
                        let mut res = T::new();
                        res.set_time(time).set_value(val.to_string());
                        return Ok(res);
                    }
                }
                return Err(StatusCode::BadNotReadable);
            },
            Err(err) => {
                drop(session);
                drop(guard);
                self.gain_new_session();
                Err(err)
            },
        }
    }

    fn write_single_retry(&self, node_id: &NodeId, value: Variant, times:u32) -> Result<(), StatusCode>{
        let mut res = self.write_single(node_id, value.clone());
        for _ in 0..times {
            if res.is_ok() { break; }else{
                res = self.write_single(node_id, value.clone());
            }
        }
        return res;
    }

    fn write_single(&self, node_id: &NodeId, value: Variant) -> Result<(),StatusCode>{
        let guard = self.session.read().unwrap();
        let session = guard.as_ref().unwrap().read();
        let value = DataValue::from(value);
        let write_value = WriteValue {
            node_id: node_id.clone(),
            attribute_id: AttributeId::Value as u32,
            index_range: UAString::null(),
            value,
        };
        let write_values = vec![write_value];
        match session.write(&write_values){
            Ok(res) if res[0].is_good() => {
                    debug_println!("Write operation success");
                    return Ok(());
            },
            Err(e) => {
                debug_println!("Write operation failed");
                drop(session);
                drop(guard);
                self.gain_new_session();
                return Err(e);
            },
            Ok(res) => {
                debug_println!("Write this specific node failed");
                return Err(res[0])
            },
        }
    }

    async fn async_write_single_retry(session: Arc<OpcuaSession>,node_id: NodeId, value: Variant, times:u32) -> Result<(),StatusCode>{
        task::spawn_blocking(move||{
            session.write_single_retry(&node_id, value, times)
        }).await.unwrap()
    }

}

//Interface exposed
impl OpcuaSession {
    pub async fn new_arc(endpoint_url:String) -> Arc<OpcuaSession> {
        Arc::new(OpcuaSession::new(&endpoint_url).await)
    }

    pub async fn new(endpoint_url:&str) -> OpcuaSession {
        let res = OpcuaSession{
            session: Arc::new(std::sync::RwLock::new(None)),
            url: endpoint_url.to_string(),
        };
        tokio::task::spawn_blocking(move ||{
            res.gain_new_session();
            return res;
        }).await.unwrap()
    }

    //Only when target and val provided matches, then write operation will performe actually.
    pub async fn async_write(session: Arc<OpcuaSession>, target: &str, val: String)->Result<(), StatusCode>{
        let config = get_node_config().await;
        let node_id = config.get_node(target);
        let variant = config.get_variant(target,val);
        if let (Some(id),Some(val)) = (node_id,variant) {
            Self::async_write_single_retry(session,id,val,5).await
        }else{ Err(StatusCode::BadNodeIdUnknown) }
    }

    pub async fn async_write_once(endpoint_url:String,instruction: Box<dyn InstructionInfo + Send + Sync>)->Result<(), StatusCode>{
        let session = OpcuaSession::new_arc(endpoint_url).await;
        Self::async_write(session, &instruction.get_target(), instruction.get_value()).await
    }

    pub async fn async_read_once<T>(endpoint_url:String,target: &str) -> Result<T,StatusCode>
    where T: 'static + Clone + StoreValueTime + Send + Sync
    {
        let session = OpcuaSession::new_arc(endpoint_url).await;
        Self::async_read(session, target).await
    }

    pub async fn async_read<T>(session: Arc<OpcuaSession>, target: &str) -> Result<T,StatusCode>
    where T: 'static + Clone + StoreValueTime + Send + Sync
    {
        let config = get_node_config().await;
        if let Some(id) = config.get_node(target){
            task::spawn_blocking(move || {session.read_single(&id)}).await.unwrap()
        }else{
            Err(StatusCode::BadNodeIdUnknown)
        }
    }

    //read multiple nodes with one try
    pub async fn async_read_batch(session: Arc<OpcuaSession>, target: &[String]) -> Result<Vec<DataTime>,StatusCode>
    {
        let config = get_node_config().await;
        let node_ids = target.into_iter().map(|v|{config.get_node(v)}).collect::<Vec<Option<NodeId>>>();
        if node_ids.contains(&None) {
            Err(StatusCode::BadNodeIdInvalid)
        }else{
            let node_ids = node_ids.into_iter().map(|v| v.unwrap()).collect::<Vec<NodeId>>();
            match task::spawn_blocking(move || {session.read_batch(node_ids)}).await {
                Ok(res) => res,
                Err(_e) => Err(StatusCode::BadOutOfMemory),
            }
        }
    }

}









