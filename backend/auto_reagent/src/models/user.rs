use actix_web::web;
use sqlx::{Any, MySql};
use super::entities::prelude::*;
use crate::middleware::sqlx_manager::SqlxManager;

pub async fn exist_user(info: &LoginInfo,pool: &web::Data<SqlxManager>) -> Result<LoginInfo,sqlx::Error>{
    let pool = pool.get("plc").unwrap();
    sqlx::query_as::<_,LoginInfo>("select id,username, password from admin where username=? and password=?")
        .bind(&info.username)
        .bind(&info.password)
        .fetch_one(pool)
        .await
}
