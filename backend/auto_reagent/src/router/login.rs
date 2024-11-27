use actix_web::{get, post, web, HttpRequest, HttpResponse};
use chrono::Duration;
use crate::middleware::{redis_data::RedisState,sqlx_manager::SqlxManager};
use crate::service::user_control::*;
use crate::service::token_control::*;
use crate::middleware::{limiter, redis_data};
//use crate::interface::user::{UserCrud,UserInfoMan};
use crate::model::user::{UserInfo,LoginInfo};

#[post("/useradd")]
async fn useradd(_req: HttpRequest,info: web::Json<UserInfo>, pool:web::Data<SqlxManager>) -> HttpResponse {
    let info = info.into_inner();
    match add_user(&info, &pool, "user").await{
        Ok(_) => HttpResponse::Ok().body("Ok"),
        Err(e) => HttpResponse::Ok().body(e.to_string()),
    }
}

#[post("/login")]
async fn login(req: HttpRequest,info: web::Json<LoginInfo>, pool:web::Data<SqlxManager>, redis_conn: web::Data<RedisState>) -> HttpResponse {
    // Add limiter from hacking
    if !limiter::rate_limit_check(&req, &redis_conn,3).await {
        return HttpResponse::Unauthorized().body("Too many requests");
    }
    // Convert into UserInfo
    let info :UserInfo = info.into_inner().into();
    match exist_user(&info, &pool,"user").await{
        Ok(val) if val => {
            let verify_interval = Duration::hours(24);
            let token = generate_token(&info,verify_interval);
            let _ = add_token(&token,3600*24,&redis_conn).await;
            let header = ("token",token);
            HttpResponse::Ok()
                .insert_header(header)
                .body("Ok")
        },
        _ => HttpResponse::Unauthorized().body("Bad User Info"),
    }
}

#[get("/verify")]
async fn check_privilege(req: HttpRequest,redis_data: web::Data<RedisState>) -> HttpResponse {
    let res = verify(&req,&redis_data).await;
    if let Some(_) = res {
        HttpResponse::Ok().json("Ok")
    }else {
        HttpResponse::Unauthorized().body("Bad token")
    }
}


