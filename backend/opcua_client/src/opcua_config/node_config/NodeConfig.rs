use std::{collections::HashMap, str::FromStr};
use opcua::types::NodeId;
use serde::Deserialize;
use super::data_type::DataType;
use opcua::types::Variant;
use tokio::sync::OnceCell as TokioOnceCell;
use std::sync::Arc;

static CONFIG: TokioOnceCell<Arc<NodeConfig>> = TokioOnceCell::const_new();

pub async fn get_node_config() -> Arc<NodeConfig> {
    CONFIG.get_or_init(|| async {
        Arc::new(NodeConfig::new().await)
    }).await.clone()
}

#[derive(Deserialize, Debug, Clone)]
pub struct Mapping {
    tag: String,
    node: String,
    dtype: Option<DataType>,
}


#[derive(Deserialize, Debug)]
pub struct NodeConfig{
    produce: Option<Vec<Mapping>>,
    test: Option<Vec<Mapping>>,
    node_built: Option<HashMap<String,(NodeId,Option<DataType>)>>
}

impl NodeConfig {
    pub async fn new() -> NodeConfig{
        //This unwrap only happens at init stage, so it is safe
        let res = tokio::task::spawn_blocking(move||{
            dotenvy::dotenv().unwrap();
            let path = dotenvy::var("NODE_CONFIG").expect("No config.yml provided or error path of config.yml");
            let content = std::fs::read_to_string(path).unwrap();
            serde_yml::from_str::<NodeConfig>(&content).unwrap()
        }).await.unwrap();
        Self::init_node_store(res)
    }

    //Init node_built, then release the memory of Mapping vector
    fn init_node_store(mut config: NodeConfig) -> NodeConfig{
        config.node_built = Some(HashMap::new());
        if let Some(ref produce) = config.produce {
            let empty :Vec<Mapping>= vec![];
            let iters = if config.test.is_none() {
                produce.into_iter().chain(empty.iter())
            }else{
                produce.into_iter().chain(config.test.as_ref().unwrap().iter()) 
            };
            iters.map(|val|{
                //This unwrap check the config syntax
                let id = NodeId::from_str(&val.node).unwrap();
                config.node_built.as_mut().unwrap().insert(val.tag.clone(),(id,val.dtype.clone()));
            }).last();
        }
        config.produce = None;
        config.test = None;
        config
    }
    
    /// Return a DataType specified with the tag
    /// None if no such tag in config.yml
    pub fn get_type(&self,tag:&str) -> Option<DataType> {
        let res = self.node_built.as_ref().unwrap().get(tag);
        match res{
            Some(res)  => { res.1.clone() },
            _ => None,
        }
    }
                    
    //this unwrap is safe due to it has been initialized
    /// None if no such tag in config.yml
    pub fn get_node(&self,tag:&str) -> Option<NodeId> {
        let res = self.node_built.as_ref().unwrap().get(tag);
        match res {
            Some(res) => return Some(res.0.clone()),
            _ => None,
        }
    }

    /// Get a variant with the same type of specific tag, with specified value set.
    /// None if no such tag in config.yml
    pub fn get_variant(&self,tag:&str,val:String) -> Option<Variant>{
        if let Some(dt) = Self::get_type(self, tag){
            match dt {
                DataType::Boolean => {
                    if let Ok(val)= val.parse::<bool>(){return Some(Variant::Boolean(val))}
                },
                DataType::Double => {
                    if let Ok(val)= val.parse::<f64>(){  return Some(Variant::Double(val))}
                },
                DataType::Float => {
                    if let Ok(val)= val.parse::<f32>(){  return Some(Variant::Float(val))}
                },
                DataType::Int16 => {
                    if let Ok(val)= val.parse::<i16>(){  return Some(Variant::Int16(val))}
                },
                DataType::Int32 => {
                    if let Ok(val)= val.parse::<i32>(){  return Some(Variant::Int32(val))}
                },
                DataType::Int64 => {
                    if let Ok(val)= val.parse::<i64>(){  return Some(Variant::Int64(val))}
                },
                DataType::UInt16 => {
                    if let Ok(val)= val.parse::<u16>(){  return Some(Variant::UInt16(val))}
                },
                DataType::UInt32 => {
                    if let Ok(val)= val.parse::<u32>(){  return Some(Variant::UInt32(val))}
                },
                DataType::UInt64 => {
                    if let Ok(val)= val.parse::<u64>(){  return Some(Variant::UInt64(val))}
                },
            };
        }
        None
    }
    
}


