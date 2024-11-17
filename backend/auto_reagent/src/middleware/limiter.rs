use actix_web::HttpRequest;
use crate::middleware::redis_data::RedisState;

pub async fn rate_limit_check(req: &HttpRequest, redis_conn: &RedisState) -> bool {
    let connection_info = req.connection_info();
    let client_ip = connection_info.realip_remote_addr().unwrap_or("unknown");
    let key = format!("rate_limit:{}", client_ip);
    let limit: u32 = 60; // Max requests per minute

    let count = redis_conn.get::<u32>(&key).await.unwrap_or(0);

    if count >= limit {
        return false; // Rate limit exceeded
    }
    let _ = redis_conn.incr(&key).await;
    let _ = redis_conn.expire(&key, 60).await; // Set 1 minute expiration
    true
}
