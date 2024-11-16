use std::collections::HashMap;
use sqlx::{any::AnyPoolOptions, Any, MySql, Pool,mysql::MySqlPoolOptions};

#[derive(Clone,Debug,Default)]
pub struct SqlxManager {
    databases: HashMap<String,Pool<MySql>>
}

impl SqlxManager {
    pub fn new() -> SqlxManager {
        SqlxManager { databases: HashMap::new() }
    }
    pub async fn add_database(&mut self,name: &str,url: String){
        // let pool = AnyPoolOptions::new().connect(&url).await.unwrap();

        let pool = MySqlPoolOptions::new().connect(&url).await.unwrap();
        self.databases.insert(name.to_string(), pool);
    }
    pub fn get(&self,db_name:&str) -> Option<&Pool<MySql>>{
        self.databases.get(db_name)
    }
}
