use actix_web::{get, post, web, HttpRequest, HttpResponse};
use chrono::{Duration, Utc};
use crate::models::entities::prelude::LoginInfo;
use super::verify_token::{verify,generate_token};
use crate::middleware::{redis_data::RedisState,sqlx_manager::SqlxManager};
use crate::models::user::exist_user;
use crate::models::token::{del_token,add_token};
use crate::middleware::limiter;

#[post("/login")]
async fn login(req: HttpRequest,info: web::Json<LoginInfo>, pool:web::Data<SqlxManager>, redis_conn: web::Data<RedisState>) -> HttpResponse {
    if !limiter::rate_limit_check(&req, &redis_conn).await {
        return HttpResponse::Unauthorized().body("Too many requests");
    }
    let res = exist_user(&info, &pool).await;
    if res.is_ok() {
        let verify_interval = Duration::hours(24);
        let user_info = info.into_inner();
        let token = generate_token(&user_info,verify_interval);
        let header = ("token",token);
        HttpResponse::Ok()
            .insert_header(header)
            .body("Ok")
    }else{
        HttpResponse::Unauthorized().body("Bad User Info")
    }
}

#[get("/verify")]
async fn check_privilege(req: HttpRequest) -> HttpResponse {
    let res = verify(&req).await;
    if let Some(res) = res {
        HttpResponse::Ok().json(res.username)
    }else {
        HttpResponse::Unauthorized().body("Bad token")
    }
}


