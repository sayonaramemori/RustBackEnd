use crate::interface::crud::SqlxCrud;
use sqlx;
use sqlx::Pool;
use sqlx::MySql;

pub trait UserInfoMan {
    fn get_username(&self) -> String{ String::from("")}
    fn get_password(&self) -> String{ String::from("")}
    fn get_email(&self) -> String{ String::from("")}
    fn ref_username(&self) -> &str{ "" }
    fn ref_password(&self) -> &str{ "" }
    fn ref_email(&self) -> &str{ "" }
    fn set_username(&mut self, _new_val:String){}
    fn set_password(&mut self, _new_val:String){}
    fn set_email(&mut self, _new_val:String){}
}

pub trait UserCrud: SqlxCrud+UserInfoMan {
    async fn create_user(&self,pool: &Pool<MySql>) -> Result<(),sqlx::Error>{Self::create(self,pool).await}
    async fn delete_user(&self,pool: &Pool<MySql>) -> Result<(),sqlx::Error>{Self::delete(self,pool).await}
    async fn update_password(&self,pool: &Pool<MySql>)-> Result<(),sqlx::Error>{Self::update(self,pool).await}
    async fn update_email(&self,pool: &Pool<MySql>)-> Result<(),sqlx::Error>{Self::update(self,pool).await}
    async fn read_user(&self,pool: &Pool<MySql>)-> Result<Self,sqlx::Error>{Self::read(self,pool).await}
}
