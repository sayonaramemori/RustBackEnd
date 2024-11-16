use std::collections::HashMap;
use actix_web::{post,get, web, Responder, HttpResponse,HttpRequest};
use crate::models::entities::prelude::*;
use crate::middleware::{redis_data::RedisState,sqlx_manager::SqlxManager};
use super::verify_token::verify;
use std::str::FromStr;

/// Return the datas in the image
#[get("/state")]
async fn turbine_state(req: HttpRequest, redis_data: web::Data<RedisState>) -> HttpResponse {
    let res = verify(&req,).await;
    if res.is_some() {
        match redis_data.hgetall::<HashMap<String,String>>(vec!["turbineState:001","turbineState:002"]).await {
            Ok(res) => {
                let mut response:Vec<TurbineState> = vec![];
                for i in res {
                    if !i.is_empty() {
                        response.push(TurbineState::to_turbine_state(i));
                    }
                }
                return HttpResponse::Ok().json(response);
            },
            Err(e) => return HttpResponse::InternalServerError().json(format!("{e} leading operation fail")),
        }
    }
    HttpResponse::Unauthorized().json("Bad Token")
}

/// Return the newest data with the specific number
#[get("/findlastVice/{num}")]
pub async fn findlast_vice(req: HttpRequest,num: web::Path<f64>,redis_data:web::Data<RedisState>,) -> impl Responder {
    return findlast_record(req, num, redis_data,  "fluxVice").await;
}

/// Return the newest data with the specific number
#[get("/findlast/{num}")]
pub async fn findlast(req: HttpRequest,num: web::Path<f64>,redis_data:web::Data<RedisState>,) -> impl Responder {
    return findlast_record(req, num, redis_data,  "flux").await;
}

/// Return the newest status of the specific target
#[get("/pumpStatus/{num}")]
pub async fn pump_status(num:web::Path<u32>,req:HttpRequest,redis_data:web::Data<RedisState>) -> HttpResponse {
    let res = verify(&req,).await;
    if res.is_some() {
        let number = num.into_inner();
        let obj = match number {
            0 => "switchStatus",
            1 => "switchViceStatus",
            2 => "setpointStatus",
            3 => "setpointViceStatus",
            _ => return HttpResponse::BadRequest().json("Bad params"),
        };
        match redis_data.get::<String>(obj).await {
            Ok(res) => return HttpResponse::Ok().json(res),
            Err(e) => return HttpResponse::InternalServerError().json(e.to_string() + " leading Operation failed"),
        }
    }
    HttpResponse::Unauthorized().json("Bad Token")
}

async fn findlast_record(req: HttpRequest,num: web::Path<f64>,redis_data:web::Data<RedisState>,target:&'static str) -> impl Responder {
    let res = verify(&req,).await;
    if res.is_some() {
        let num = num.into_inner() as i64;
        match redis_data.lrange(target, num as usize).await {
            Ok(res) => {
                let res :Vec<TempRecord<String>> = res.into_iter()
                    .filter_map(|v| Record::from_str(&v).ok())
                    .zip(0..)
                    .map(|(v,i)|{ (v,i).into() })
                    .collect();
                return HttpResponse::Ok().json(res);
            },
            Err(e) => return HttpResponse::InternalServerError().json(e.to_string()+" leading operation fail"),
        };
    }else{
        HttpResponse::Unauthorized().json("Bad token")
    }
}


