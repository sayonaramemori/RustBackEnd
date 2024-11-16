extern crate opcua_client;
use opcua_client::example::auto_reagent::*;
use tokio;

#[tokio::main]
async fn main() {
    match do_record().await {
        Ok(_) => {},
        Err(_e) => {}
    }
}

