use serde::{Deserialize, Serialize};
use sqlx;
#[derive(Deserialize,sqlx::FromRow,Debug,Serialize,PartialEq,Eq)]
pub struct LoginInfo {
    pub username: String,
    pub password: String,
}