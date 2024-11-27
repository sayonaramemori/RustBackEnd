use actix_web::web;
use redis::RedisError;
use crate::middleware::redis_data::RedisState;
use actix_web::HttpRequest;
use crate::model::user::UserInfo;
use crate::debug_println;
use crate::middleware::token_man::TokenManager;


pub fn generate_token(info: &UserInfo,expire_time: chrono::TimeDelta) -> String {
    TokenManager::new().generate_token_with_time(info,expire_time)
}

pub async fn verify(req: &HttpRequest, redis_data: &web::Data<RedisState>) -> Option<UserInfo>{ 
    let cinfo = req.connection_info();
    let ip = cinfo.realip_remote_addr().unwrap_or("unknown");
    let token = match req.headers().get("token") {
        Some(header_value) => header_value.to_str().unwrap_or(""),
        _ => "",
    };
    if token.is_empty() {
        debug_println!("No token provided");
        return None;
    }
    let token_man = TokenManager::new();
    if let Some(info) = token_man.unravel_with_time_check::<UserInfo>(token){
        debug_println!("Query in Redis with ip: {}",ip); 
        return match exist_token(token,redis_data).await {
            Ok(temp) if !temp.is_empty() => { Some(info) },
            _ => None,
        }
    }
    None
}

pub async fn del_token(token:&str,redis_data: &web::Data<RedisState>,)->Result<bool,RedisError> {
    redis_data.del(token).await
}

pub async fn add_token(token:&str,sec: u32, redis_data: &web::Data<RedisState>,) ->Result<(),RedisError>{
    redis_data.setex(token, 1, sec).await
}

pub async fn exist_token(token:&str,redis_data: &web::Data<RedisState>,) ->Result<String,RedisError>{
    redis_data.get(token).await
}
