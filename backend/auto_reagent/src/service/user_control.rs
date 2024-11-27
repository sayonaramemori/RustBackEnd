use actix_web::web;
use crate::middleware::sqlx_manager::SqlxManager;
use crate::utility::pass_hash::*;
use crate::interface::{user::UserInfoMan,user::UserCrud};
use sqlx;


/// Caution for SQL 
pub async fn exist_user<T: UserInfoMan + UserCrud>(info: &T,pool: &web::Data<SqlxManager>, db_key: &str) -> Result<bool,sqlx::Error>{
    let pool = pool.get(db_key).unwrap();
    // Query via username
    match info.read_user(pool).await {
        Ok(user) => {
            let password_hashed = user.ref_password();
            Ok(verify_password(info.ref_password(),password_hashed))
        },
        Err(e) => Err(e),
    }
}

pub async fn add_user<T: UserInfoMan + UserCrud>(info: &T, pool: &web::Data<SqlxManager>, db_key: &str) -> Result<(),sqlx::Error>{
    let pool = pool.get(db_key).unwrap();
    // Create the table if not exits
    let _ = info.create_table_if_not_exist(pool).await;
    // Hash the plain text password
    let password_hashed = hash_password(info.ref_password()).unwrap();
    // Create a new user with password_hashed
    let mut info = info.clone();
    info.set_password(password_hashed);
    match info.create_user(pool).await{
       Ok(_) => Ok(()),
       Err(e) => Err(e),
    }
}
