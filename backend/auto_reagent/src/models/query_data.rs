use actix_web::web;
use chrono::prelude::*;
use sqlx::{pool, Any, MySql};
use crate::middleware::sqlx_manager::SqlxManager;
use crate::models::entities::prelude::*;
use crate::handlers::entities::date_time_range::*;

/// Query the datas between the given time range.
pub async fn get_data_in_range(pool: &web::Data<SqlxManager>,time_pair:(NaiveDateTime,NaiveDateTime),db_name:&str,table:&str) -> Result<Vec<TempRecord<DateTime<Utc>>>,sqlx::Error> {
    let pool = pool.get(db_name).unwrap();
    let query = format!("SELECT val,id,time FROM {table} WHERE time BETWEEN ? AND ?");
    //Any feature currently not support chrono.
    sqlx::query_as::<_,TempRecord<DateTime<Utc>>>(&query)
        .bind(time_pair.0)
        .bind(time_pair.1)
        .fetch_all(pool)
        .await
}

