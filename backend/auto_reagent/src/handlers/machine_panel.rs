use super::verify_token::verify;
use actix_web::{get, web,  HttpResponse,HttpRequest};
use crate::middleware::{myws::*,redis_data::RedisState};

use actix::prelude::*;
use std::sync::{RwLock,Arc};

#[get("/startMain")]
pub async fn start_main(data: web::Data<Arc<RwLock<Vec<Addr<MyWs>>>>>,req:HttpRequest) -> HttpResponse {
    let res = verify(&req).await;
    if res.is_some() {
        let target = "switch";
        return start(target.to_string(),data).await;
    }
    HttpResponse::Unauthorized().json("Bad Token")
}

#[get("/startVice")]
pub async fn start_vice(data: web::Data<Arc<RwLock<Vec<Addr<MyWs>>>>>,req:HttpRequest) -> HttpResponse {
    let res = verify(&req).await;
    if res.is_some() {
        let target = "switchVice";
        return start(target.to_string(),data).await;
    }
    HttpResponse::Unauthorized().json("Bad Token")
}

#[get("/stopMain")]
pub async fn stop_main(data: web::Data<Arc<RwLock<Vec<Addr<MyWs>>>>>,req:HttpRequest) -> HttpResponse {
    let res = verify(&req).await;
    if res.is_some() {
        let target = "switch";
        return stop(target.to_string(),data).await;
    }
    HttpResponse::Unauthorized().json("Bad Token")
}

#[get("/stopVice")]
pub async fn stop_vice(data: web::Data<Arc<RwLock<Vec<Addr<MyWs>>>>>,req:HttpRequest) -> HttpResponse {
    let res = verify(&req).await;
    if res.is_some() {
        let target = "switchVice";
        return stop(target.to_string(),data).await;
    }
    HttpResponse::Unauthorized().json("Bad Token")
}


#[get("/setpoint/{num}/{sp}")]
pub async fn set_point(data: web::Data<Arc<RwLock<Vec<Addr<MyWs>>>>>,nums:web::Path<(u32,f64)>,req:HttpRequest) -> HttpResponse{
    let res = verify(&req).await;
    if res.is_some() {
        let (num,sp) = nums.into_inner();
        let obj = match num {
            0 => "setpoint",
            1 => "setpointVice",
            _ => return HttpResponse::BadRequest().json("Bad params"),
        };
        if sp<0.0 || sp>200.0 {return HttpResponse::BadRequest().json("Bad params");}
        return setpoint(obj.to_string(),sp.to_string(),data).await;
    }
    HttpResponse::Unauthorized().json("Bad Token")
}
