use actix_web::web;
use redis::RedisError;
use crate::middleware::redis_data::RedisState;

pub async fn del_token(token_key:&str,redis_data: &web::Data<RedisState>,)->Result<bool,RedisError> {
    redis_data.del(token_key).await
}

pub async fn add_token(key:&str,token:&str,sec: u32, redis_data: &web::Data<RedisState>,) ->Result<(),RedisError>{
    redis_data.setex(key, token, sec).await
}

pub async fn exist_token(token_key:&str,redis_data: &web::Data<RedisState>,) ->Result<String,RedisError>{
    redis_data.get(token_key).await
}