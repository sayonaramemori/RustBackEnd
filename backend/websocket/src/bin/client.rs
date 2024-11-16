use tokio_tungstenite::connect_async;

use tokio_tungstenite::tungstenite::protocol::Message;
use std::{sync::Arc, time::Duration};
use tokio::time::sleep;

use tokio_tungstenite::MaybeTlsStream;
use tokio_tungstenite::WebSocketStream;
use tokio::net::TcpStream;
use futures::stream::{StreamExt,SplitStream,SplitSink};
use futures::SinkExt;

use AutoReagent::middleware::redis_data::RedisState as RedisData;
use opcua_client::opcua_config::data_adaptor::unit::Instruction::Instruction;
use opcua_client::opcua_config::opcua_session_wrapper::OpcuaSession;
// use client_test::opcua_config::data_adaptor::interface::transfer::InstructionInfo;
use opcua_client::debug_println;




#[tokio::main]
async fn main() {
    let wait_time = Duration::from_secs(5);
    loop {
        match connect_to_server().await {
            Ok(_) => { },
            Err(e) => {
                eprintln!("Fail to Connect for {e}");
                sleep(wait_time).await;
            }
        }
    }
}

async fn connect_to_server() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().unwrap();
    let url = dotenvy::var("WEBSOCKET_URL").unwrap();
    let (ws_stream, _) = connect_async(url).await?;
    let (write, read) = ws_stream.split();
    let read_handle = tokio::spawn(handle_read(read));
    let write_handle = tokio::spawn(keep_alive(write));
    debug_println!("Connect func called");

    if let Err(e) = tokio::try_join!(read_handle, write_handle) {
        eprintln!("Error in WebSocket communication: {:?}", e);
    }

    Ok(())
}

async fn handle_read(mut read:SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>){
        let redis_passwd:String = dotenvy::var("REDIS_PASSWD").unwrap();
        let redis_url:String = dotenvy::var("REDIS_URL").unwrap();
        let redis_data = RedisData::new_arc(redis_passwd,redis_url);
        while let Some(message) = read.next().await {
            match message {
                Ok(Message::Text(text)) => {
                    let instruction = serde_json::from_str::<Instruction>(&text);
                    match instruction {
                        Ok(res) => {
                            handle_instruction(redis_data.clone(),res).await
                        },
                        Err(_) => debug_println!("Not a instruction"),
                    }
                },
                Ok(Message::Ping(_)) => {},
                Ok(Message::Pong(_)) => {},
                Ok(Message::Frame(_)) => {},
                Ok(Message::Binary(_)) => {},
                Ok(Message::Close(_e)) => { debug_println!("Closed");},
                Err(e) => eprintln!("Error message for {}",e),
            };
        }
        debug_println!("receive-task over");
}

//Send Ping every 15 second  
async fn keep_alive(mut write:SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>){
    while let Ok(_) = write.send(Message::Ping(Vec::new())).await { sleep(std::time::Duration::from_secs(15)).await; }
    debug_println!("send task over");
}


async fn handle_instruction(redis_data:Arc<RedisData>,instruction: Instruction) {
    debug_println!("Receive {:?}",instruction);
    let endpoint_url:String = dotenvy::var("OPCUA_URL").unwrap();
    if let Ok(_) = OpcuaSession::async_write_once(endpoint_url, Box::new(instruction.clone())).await {
        let status_key = format!("{}Status",instruction.target);
        redis_data.setex_retry(&status_key, instruction.value,10,5).await;
    }
}