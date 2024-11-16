use actix_web::{get, guard, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use serde_json;
use crate::handlers::{login::*,monitor::*};
use std::sync::{RwLock,Arc};
use actix_web_actors::ws;
use actix::prelude::*;
use serde::{Deserialize,Serialize};
use actix::{Actor, StreamHandler};

#[derive(Message, Deserialize,Clone,Serialize)]
#[rtype(result = "()")]
pub struct Instruction {
    pub target: String,
    pub value: String,
}

async fn operation_on(ins:Instruction,data: web::Data<Arc<RwLock<Vec<Addr<MyWs>>>>>,)->HttpResponse{
    let mut guard = data.write().unwrap();
    guard.retain(|x| x.connected());
    println!("Alive size: {}",guard.len());
    if guard.is_empty() {
        HttpResponse::InternalServerError().body("No WebSocket connection")
    }else{
        for addr in guard.iter(){
            addr.do_send(ins.clone());
        }
        HttpResponse::Ok().body("Instruction sent")
    }
}

pub async fn start(target:String,data: web::Data<Arc<RwLock<Vec<Addr<MyWs>>>>>,)->HttpResponse {
    let ins = Instruction{target,value:"true".to_string()};
    return operation_on(ins, data).await;
}

pub async fn stop(target:String,data: web::Data<Arc<RwLock<Vec<Addr<MyWs>>>>>,)->HttpResponse {
    let ins = Instruction{target,value:"false".to_string()};
    return operation_on(ins, data).await;
}

pub async fn setpoint(target:String,value:String,data: web::Data<Arc<RwLock<Vec<Addr<MyWs>>>>>,)->HttpResponse {
    let ins = Instruction{target,value};
    return operation_on(ins, data).await;
}

//only for postman test
#[get("/si")]
async fn send_instruction(
    instruction: web::Json<Instruction>,
    data: web::Data<Arc<RwLock<Vec<Addr<MyWs>>>>>,
) -> HttpResponse {
    let mut guard = data.write().unwrap();
    guard.retain(|x| x.connected());
    println!("Alive size: {}",guard.len());
    if guard.is_empty() {
        HttpResponse::InternalServerError().body("No WebSocket connection")
    }else{
        let ins = instruction.into_inner();
        for addr in guard.iter(){
            addr.do_send(ins.clone());
        }
        HttpResponse::Ok().body("Instruction sent")
    }
}
pub struct MyWs;
impl Actor for MyWs { type Context = ws::WebsocketContext<Self>; }

//handle receive
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx : &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                //do nothing for in coming msg
                println!("Received: {}", text);
                // ctx.text(format!("Echo: {}", text));
            },
            Ok(ws::Message::Binary(binary)) => ctx.binary(binary),
            Ok(ws::Message::Ping(ping)) => {
                // ctx.ping(&ping)
            },
            Ok(ws::Message::Pong(pong)) => ctx.pong(&pong),
            Ok(ws::Message::Close(_)) => { },
            _ => {
                println!("Ji le");
            }
        }
    }
}

//handle send
impl Handler<Instruction> for MyWs {
    type Result = ();
    fn handle(&mut self, msg: Instruction, ctx: &mut Self::Context) {
        let response = format!("target: {}, Value: {}", msg.target, msg.value);
        println!("{response}");
        let res = serde_json::to_string(&msg).unwrap();
        ctx.text(res);
    }
}

#[get("/ws")]
async fn websocket_index(req: HttpRequest, stream: web::Payload, addr: web::Data<Arc<RwLock<Vec<Addr<MyWs>>>>>) -> Result<HttpResponse,Error> {
    let res = ws::WsResponseBuilder::new(MyWs, &req, stream).start_with_addr();
    let addr = addr.into_inner();
    let mut guard =  addr.write().unwrap();
    match res {
        Ok(res) => {
            let (addr,response) = res;
            guard.push(addr);
            return Ok(response);
        },
        Err(e) => Err(e),
    }
}