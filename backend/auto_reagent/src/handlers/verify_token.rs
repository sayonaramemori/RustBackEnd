use actix_web::{web, HttpRequest};
use crate::models::entities::prelude::*;
use crate::debug_println;
use crate::middleware::token_man::TokenManager;


pub fn generate_token(info: &LoginInfo,expire_time: chrono::TimeDelta) -> String {
    TokenManager::new().generate_token_with_time(info,expire_time)
}

pub async fn verify(req: &HttpRequest) -> Option<LoginInfo>{ 
    let cinfo = req.connection_info();
    let ip = cinfo.realip_remote_addr().unwrap_or("unknown");
    println!("{ip}");
    let token = match req.headers().get("token") {
        Some(header_value) => header_value.to_str().unwrap_or(""),
        _ => "",
    };
    if token.is_empty() {
        debug_println!("No token provided");
        return None;
    }
    let token_man = TokenManager::new();
    if let Some(info) = token_man.unravel_with_time_check::<LoginInfo>(token){
        debug_println!("Query in Redis"); 
        return Some(info);
    }
    None
}
