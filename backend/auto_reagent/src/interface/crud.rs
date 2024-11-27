use sqlx;
use sqlx::Pool;
use sqlx::MySql;

pub trait SqlxCrud: Sized + Clone {
    async fn create(&self,_pool: &Pool<MySql>) -> Result<(),sqlx::Error>{Ok(())}
    async fn read(&self,_pool: &Pool<MySql>) -> Result<Self,sqlx::Error>{Ok(self.clone())}
    async fn update(&self,_pool: &Pool<MySql>) -> Result<(),sqlx::Error>{Ok(())}
    async fn delete(&self,_pool: &Pool<MySql>) -> Result<(),sqlx::Error>{Ok(())}
    async fn create_table_if_not_exist(&self,_pool: &Pool<MySql>) -> Result<(),sqlx::Error>{Ok(())}
}

