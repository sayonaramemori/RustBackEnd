use serde::Deserialize;
use std::collections::HashMap;
#[derive(Deserialize,Debug,Clone)]
pub struct Config{
    pub port: String,
    // 0.0.0.0 or localhost
    pub host: String,
    pub database_url: HashMap<String,String>,
    pub redis_url: String,
    pub redis_password: String,
}

use std::path::PathBuf;
impl Config{
    pub fn init(pb:&PathBuf)->Config{
        let content = std::fs::read_to_string(pb).expect("No such config file");
        let res = serde_yml::from_str(&content).expect("Parse config.yml failed");
        res
    }
}
