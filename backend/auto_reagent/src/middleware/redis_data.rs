use std::{fmt::Display, sync::Arc};
use redis::{aio::MultiplexedConnection,  FromRedisValue, RedisResult};

#[derive(Clone)]
pub struct RedisState {
    pub redis_client: Arc<redis::Client>,
    pub redis_passwd: String,
}

impl RedisState {
    pub fn new_arc(pass:String,url:String) -> Arc<RedisState> {
        let redis_data = Self::new(pass,url);
        Arc::new(redis_data)
    }

    pub fn new(pass:String,url: String) -> RedisState{
        let redis_client = redis::Client::open(url).unwrap();
        return RedisState{redis_client: Arc::new(redis_client),redis_passwd: pass}
    }

    pub async fn llen(&self,key:&str) -> RedisResult<i32>{
        let mut conn = self.get_auth_connection().await?;
        redis::cmd("LLEN").arg(key).query_async::<_,i32>(&mut conn).await
    }

    pub async fn get_auth_connection(&self) ->RedisResult<MultiplexedConnection> {
        let mut conn = self.redis_client.get_multiplexed_async_connection().await?;
        redis::cmd("AUTH").arg(&self.redis_passwd).query_async(&mut conn).await?;
        Ok(conn)
    }

    pub async fn lpop(&self,key:&str,count:usize) ->RedisResult<Vec<String>> {
        let mut conn = self.get_auth_connection().await?;
        redis::cmd("LPOP").arg(key).arg(count).query_async::<_,Vec<String>>(&mut conn).await
    }

    pub async fn rpop(&self,key:&str,count:usize) ->RedisResult<Vec<String>> {
        let mut conn = self.get_auth_connection().await?;
        redis::cmd("RPOP").arg(key).arg(count).query_async::<_,Vec<String>>(&mut conn).await
    }

    pub async fn set(&self,key:&str,val:String) ->RedisResult<()> {
        let mut conn = self.get_auth_connection().await?;
        redis::cmd("SET").arg(key).arg(val).query_async::<_,()>(&mut conn).await
    }

    pub async fn setex<T>(&self,key:&str,val:T,sec:u32) ->RedisResult<()> 
    where T: ToString
    {
        let mut conn = self.get_auth_connection().await?;
        redis::cmd("SETEX").arg(key).arg(sec.to_string()).arg(val.to_string()).query_async::<_,()>(&mut conn).await
    }

    pub async fn setex_retry<T>(&self,key:&str,val:T,sec:u32,count:usize)
    where T: Clone + Display
    {
        for _ in 0..count {
            if let Ok(_) = Self::setex(self, key, val.to_string(), sec).await {
                return;
            }
        }
    }

    pub async fn sadd(&self,key:&str,val:&str) ->RedisResult<()>{
        let mut conn = self.get_auth_connection().await?;
        redis::cmd("SADD").arg(key).arg(val).query_async::<_,()>(&mut conn).await
    }

    pub async fn sismember(&self,key:&str,val:&str) ->RedisResult<bool>{
        let mut conn = self.get_auth_connection().await?;
        redis::cmd("SISMEMBER").arg(key).arg(val).query_async::<_,bool>(&mut conn).await
    }

    pub async fn expire(&self,key:&str,sec:u64) ->RedisResult<()>{
        let mut conn = self.get_auth_connection().await?;
        redis::cmd("EXPIRE").arg(key).arg(sec).query_async::<_,()>(&mut conn).await
    }

    pub async fn get<T: FromRedisValue>(&self,key:&str) ->RedisResult<T>{
        let mut conn = self.get_auth_connection().await?;
        redis::cmd("GET").arg(key).query_async::<_,T>(&mut conn).await
    }

    pub async fn incr(&self,key:&str) ->RedisResult<()>{
        let mut conn = self.get_auth_connection().await?;
        redis::cmd("INCR").arg(key).query_async::<_,()>(&mut conn).await
    }

    pub async fn del(&self,key:&str) ->RedisResult<bool>{
        let mut conn = self.get_auth_connection().await?;
        redis::cmd("DEL").arg(key).query_async::<_,bool>(&mut conn).await
    }

    pub async fn rpush(&self,key:&str,args:Vec<String>,) ->RedisResult<()>{
        let mut conn = self.get_auth_connection().await?;
        let mut cmd = redis::cmd("RPUSH");
        cmd.arg(key);
        args.into_iter().map(|arg|{cmd.arg(arg);}).last();
        cmd.query_async::<_,()>(&mut conn).await
    }

    pub async fn rpush_retry(&self,key:&str,args:Vec<String>,count:usize) {
        for _ in 0..count {
            if let Ok(_) = Self::rpush(self,key,args.clone()).await {
                return;
            }
        }
    }

    pub async fn lpush_ex(&self,key:&str,args:Vec<String>,sec:u32) ->RedisResult<()>{
        let mut conn = self.get_auth_connection().await?;
        let mut temp = redis::pipe();
        temp.add_command(redis::cmd("LPUSH")).arg(key);
        args.into_iter().map(|arg|{temp.arg(arg);}).last();
        temp.add_command(redis::cmd("EXPIRE")).arg(key).arg(sec.to_string());
        temp.query_async::<_,()>(&mut conn).await
    }

    pub async fn lrange(&self,key:&str,count:usize) -> RedisResult<Vec<String>> {
        let mut conn = self.get_auth_connection().await?;
        redis::cmd("LRANGE").arg(key).arg("-".to_string() + &count.to_string()).arg("-1").query_async::<_,Vec<String>>(&mut conn).await
    }

    pub async fn hgetall<T>(&self,keys:Vec<&str>)  -> RedisResult<Vec<T>> 
    where T: FromRedisValue
    {
        let mut conn = self.get_auth_connection().await?;
        let mut pipe = redis::pipe();
        let cmd = redis::cmd("HGETALL");
        for key in keys {pipe.add_command(cmd.clone()).arg(key);}
        pipe.query_async::<_,Vec<T>>(&mut conn).await
    }

}
