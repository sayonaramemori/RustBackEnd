use crate::interface::{user::UserInfoMan,crud::SqlxCrud,user::UserCrud};
use serde::{Deserialize, Serialize};
use sqlx::{self, MySql};
use sqlx::Pool;
use std::convert::From;

#[derive(Deserialize,sqlx::FromRow,Debug,Serialize,PartialEq,Eq,Clone)]
pub struct UserInfo {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Deserialize,sqlx::FromRow,Debug,Serialize,PartialEq,Eq)]
pub struct LoginInfo {
    pub username: String,
    pub password: String,
}

impl From<LoginInfo> for UserInfo{
    fn from(value: LoginInfo) -> Self {
        UserInfo{
            username: value.username,
            password: value.password,
            email: String::from(""),
        }
    }
}

impl UserInfoMan for UserInfo{
    fn get_email(&self) -> String { self.email.clone() }
    fn get_username(&self) -> String { self.username.clone() }
    fn get_password(&self) -> String { self.password.clone() }
    fn ref_email(&self) -> &str { self.email.as_ref() }
    fn ref_username(&self) -> &str { self.username.as_ref() }
    fn ref_password(&self) -> &str { self.password.as_ref() }
    fn set_username(&mut self, new_val:String) { self.username = new_val; }
    fn set_email(&mut self, new_val:String) { self.email = new_val; }
    fn set_password(&mut self, new_val:String) { self.password = new_val; }
}

// Caution for all SQL statements
impl SqlxCrud for UserInfo{
    async fn create_table_if_not_exist(&self,pool: &Pool<MySql>) -> Result<(),sqlx::Error> {
        let create_table_sql = r#"
        CREATE TABLE IF NOT EXISTS User (
            id INT AUTO_INCREMENT PRIMARY KEY,
            username VARCHAR(255) NOT NULL UNIQUE,
            password VARCHAR(255) NOT NULL,
            email VARCHAR(255) NOT NULL UNIQUE,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )
        "#;
        match sqlx::query(create_table_sql).execute(pool).await{
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
    async fn read(&self,pool: &Pool<MySql>) -> Result<UserInfo,sqlx::Error> {
        let res = sqlx::query_as::<_,UserInfo>("select username, password, email from User where username=?")
            .bind(self.ref_username())
            .fetch_one(pool)
            .await;
        return res;
    }
    async fn create(&self,pool: &Pool<MySql>) -> Result<(),sqlx::Error> {
       let result = sqlx::query("INSERT INTO User (username, password, email) VALUES (?, ?, ?)")
           .bind(self.ref_username())
           .bind(self.ref_password())
           .bind(self.ref_email())
           .execute(pool)
           .await;
       match result{
           Ok(_) => Ok(()),
           Err(e) => Err(e),
       }
    }
    async fn delete(&self,pool: &Pool<MySql>) -> Result<(),sqlx::Error> {
       let result = sqlx::query("DELETE FROM User WHERE username = ?")
            .bind(self.ref_username())
            .execute(pool)
            .await;
       match result{
           Ok(_) => Ok(()),
           Err(e) => Err(e),
       }
    }
}

impl UserCrud for UserInfo{
    async fn update_email(&self,pool: &Pool<MySql>)-> Result<(),sqlx::Error> {
        let result = sqlx::query("UPDATE users SET email = ? WHERE username = ?")
            .bind(self.ref_email())
            .bind(self.ref_username())
            .execute(pool)
            .await;
       match result{
           Ok(_) => Ok(()),
           Err(e) => Err(e),
       }
    }
    async fn update_password(&self,pool: &Pool<MySql>)-> Result<(),sqlx::Error> {
        let result = sqlx::query("UPDATE users SET password= ? WHERE username = ?")
            .bind(self.ref_password())
            .bind(self.ref_username())
            .execute(pool)
            .await;
       match result{
           Ok(_) => Ok(()),
           Err(e) => Err(e),
       }
    }
}

