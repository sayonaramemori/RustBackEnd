use sqlx::mysql::MySqlPoolOptions;
use sqlx::{MySql, Pool};
use tokio::time::sleep;
use tokio::sync::broadcast::Receiver;
use mysql::*;
use AutoReagent::middleware::redis_data::RedisState as RedisData;

use crate::entity::temperature::Temperature;
use crate::debug_println;
use crate::opcua_config::data_adaptor::{collector::DataCollector,unit::DataTime::DataTime};
type MyResult<T> = Result<T,Box<dyn std::error::Error + Send + Sync>>;

/// Collect data to redis for query from frontend.
/// This funtion run in a loop, return when error occuers.
pub async fn to_redis_list(mut recv: Receiver<DataTime>,target:&'static str,) -> MyResult<()>
{
    let redis_passwd:String = dotenvy::var("REDIS_PASSWD").unwrap();
    let redis_url:String = dotenvy::var("REDIS_URL").unwrap();
    let redis_data = RedisData::new_arc(redis_passwd,redis_url);
    while let Ok(res) = recv.recv().await {
        let res = serde_json::to_string(&res).unwrap();
        redis_data.rpush_retry(target, vec![res],3).await;
    }
    Ok(())
}

/// To simply write a new value to a specific redis key.
/// This funtion run in a loop, return when error occuers.
pub async fn to_redis_str(mut recv: Receiver<DataTime>,target:&'static str,)-> MyResult<()>
{
    let redis_passwd:String = dotenvy::var("REDIS_PASSWD").unwrap();
    let redis_url:String = dotenvy::var("REDIS_URL").unwrap();
    let redis_data = RedisData::new_arc(redis_passwd,redis_url);
    while let Ok(res) = recv.recv().await {
        //only data needed
        redis_data.setex_retry(target, res.v ,9,3).await;
    }
    Ok(())
}

/// Insert data to 
pub async fn insert_data(pool: &Pool<MySql>, data: &Vec<Temperature>, sql: &String) -> MyResult<()>
{
    let mut transaction = pool.begin().await.map_err(|e| format!("Transaction error for {e}"))?;
    for entry in data {
        sqlx::query(sql)
            .bind(entry.val)
            .bind(entry.time)
            .execute(&mut transaction)
            .await
            .map_err(|e| format!("Transaction error for {e}"))?;
    }
    transaction.commit().await.map_err(|e| format!("Transaction error for {e}"))?;
    Ok(())
}

use chrono::Datelike;
fn get_table_name_prefix() -> String {
    let now = chrono::Local::now();
    let formatted_date = now.format("%Y%m%d").to_string();
    let weekday = now.weekday().num_days_from_monday() + 1;
    let result = format!("{formatted_date}_{weekday}");
    result
}

//store to database
async fn flux_to_mysql(mut recv: Receiver<DataTime>,url:String) -> MyResult<()>
{
    sleep(std::time::Duration::from_secs(20)).await;
    let table = get_table_name_prefix();
    let creat_cmd = format!("CREATE TABLE if not exists {table}(id bigint auto_increment,val double not null,time timestamp not null,primary key(id))");
    let insert_cmd = format!("INSERT INTO {table}(val,time) VALUES (?, ?)");
    let mut records:Vec<Temperature> = vec![];
    loop {
        match MySqlPoolOptions::new().connect(&url).await {
            Ok(pool) => {
                if let Ok(_) = sqlx::query::<MySql>(&creat_cmd).execute(&pool).await {
                    while let Ok(msg) = recv.try_recv(){ records.push(Temperature::from(msg)); }
                    match insert_data(&pool, &records, &insert_cmd).await {
                        Ok(_) => {
                            debug_println!("Successfully store data to MySql");
                            records.clear(); 
                        },
                        Err(e) => debug_println!("Fail to store data to MySql for {e}"),
                    }
                    sleep(std::time::Duration::from_secs(300)).await;
                }else{
                    debug_println!("Query mysql failed, try agian after 5s");
                    sleep(std::time::Duration::from_secs(5)).await;
                }
            },
            Err(e) => {
                debug_println!("Connect database failed, for {:?} try agian after 5s",e);
                sleep(std::time::Duration::from_secs(5)).await;
            }
        }
    }
}

//trim the record list to specified length with the specific time interval
async fn trim_record(max_num:i32,interval:u64,target:&'static str,)-> MyResult<()>
{
    let redis_passwd:String = dotenvy::var("REDIS_PASSWD").unwrap();
    let redis_url:String = dotenvy::var("REDIS_URL").unwrap();
    let redis_data = RedisData::new_arc(redis_passwd,redis_url);
    loop {
        match redis_data.llen(target).await {
            Ok(num) => {
                debug_println!("Redis record {target} length is {num}");
                let subtract = num - max_num;
                if subtract > 0 {
                    let _ = redis_data.lpop(target, subtract as usize).await;
                }
            },
            Err(err) => debug_println!("Trim record failed for {:?}",err),
        }
        sleep(std::time::Duration::from_secs(interval)).await;   
    }
}

async fn gain_status(collector:DataCollector<DataTime>,target:&'static str,)-> MyResult<()>{
    let j1 = tokio::spawn(to_redis_str(collector.subscribe(), target,));
    let j2 = tokio::spawn(DataCollector::execute_loop(collector));
    tokio::try_join!(j1,j2)?;
    Ok(())
}

async fn record_to_redis_mysql(collector:DataCollector<DataTime>,target:&'static str,mysql_url:String,)->MyResult<()>{
    println!("Url is {mysql_url}");
    let j1 = tokio::spawn(flux_to_mysql(collector.subscribe(),mysql_url));
    let j2 = tokio::spawn(to_redis_list(collector.subscribe(),target,));
    let j3 = tokio::spawn(trim_record(3600,600,target,));
    let j4 = tokio::spawn(DataCollector::execute_loop(collector));
    tokio::try_join!(j1,j2,j3,j4)?;
    Ok(())
}

// business
pub async fn do_record() -> MyResult<()>{
    dotenvy::dotenv().unwrap();
    let endpoint_url = dotenvy::var("OPCUA_URL").unwrap();
    let sp_colletor: DataCollector<DataTime> = DataCollector::new("setpoint",3600,&endpoint_url);
    let sp_vice_colletor: DataCollector<DataTime> = DataCollector::new("setpointVice",3600,&endpoint_url);
    let switch_colletor: DataCollector<DataTime> = DataCollector::new("switch",3600,&endpoint_url);
    let switch_vice_colletor: DataCollector<DataTime> = DataCollector::new("switchVice",3600,&endpoint_url);
    let flux_colletor: DataCollector<DataTime> = DataCollector::new("flux",3600,&endpoint_url);
    let flux_vice_colletor: DataCollector<DataTime> = DataCollector::new("fluxVice",3600,&endpoint_url);
    tokio::try_join!( 
        gain_status(sp_colletor,"setpointStatus",),
        gain_status(sp_vice_colletor,"setpointViceStatus",),
        gain_status(switch_colletor,"switchStatus",),
        gain_status(switch_vice_colletor,"switchViceStatus",),
        record_to_redis_mysql(flux_colletor,"flux",dotenvy::var("flux").unwrap(),),
        record_to_redis_mysql(flux_vice_colletor,"fluxVice",dotenvy::var("fluxVice").unwrap(),),
    )?;
    Ok(())
}