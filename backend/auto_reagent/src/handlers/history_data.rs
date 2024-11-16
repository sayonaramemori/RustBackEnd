use actix_web::{post,get, web, Responder, HttpResponse,HttpRequest};
use chrono::prelude::*;
use crate::models::entities::prelude::*;
use crate::handlers::entities::date_time_range::*;
use crate::middleware::{redis_data::RedisState,sqlx_manager::SqlxManager};
use super::verify_token::verify;
use crate::models::query_data::get_data_in_range;
use super::entities::history_data::HistoryData;
use crate::utility::time::with_timezone;
use chrono::Datelike;

#[post("/historyMain")]
async fn main_history(req: HttpRequest, pool: web::Data<SqlxManager>,data:web::Json<StringDateTimeRng>) -> HttpResponse {
    return history(req,pool,data,"flux").await;
}

#[post("/historyVice")]
async fn vice_history(req: HttpRequest, pool: web::Data<SqlxManager>,data:web::Json<StringDateTimeRng>) -> HttpResponse {
    return history(req,pool,data,"fluxVice").await;
}

/// To provide the table name -- named with ymd
fn get_table_name_prefix() -> String {
    let now = chrono::Local::now();
    let formatted_date = now.format("%Y%m%d").to_string();
    let weekday = now.weekday().num_days_from_monday() + 1;
    format!("{}_{}", formatted_date, weekday)
}


fn analysis_history_data(data: Vec<TempRecord<DateTime<Utc>>>, output_number: usize, min_interval: i64, max_interval: i64) ->HistoryData<NaiveDateTime> {
    let mut totol_timedelta :i64 = 0;
    let integration :f64= data.windows(2).map(|val|{
        let interval = val[1].time.signed_duration_since(val[0].time).num_seconds();
        let interval = if interval < min_interval {min_interval}else if interval > max_interval {max_interval} else { interval };
        totol_timedelta += interval; 
        let mean = (val[1].val + val[0].val)/2.0;
        let sub_integration = mean * (interval as f64);
        sub_integration
    }).fold(0.0, |init,x| init + x);
    let average = integration/(totol_timedelta as f64);
    let hours = (totol_timedelta as f64)/3600.0;
    let skip_step = data.len()/output_number;
    let mut res:Vec<TempRecord<NaiveDateTime>> = vec![];
    let mut iterator = data.into_iter();
    while let Some(item) = iterator.nth(skip_step) { res.push(item.into()); }
    HistoryData {
        average,
        total_time: hours,
        records: res,
    }
}

async fn history(req: HttpRequest, pool: web::Data<SqlxManager>,data:web::Json<StringDateTimeRng>,db_name:&'static str) -> HttpResponse {
    let res = verify(&req).await;
    if res.is_some() {
        if let (Ok(start),Ok(end)) = (data.start.parse::<DateTime<Utc>>(),data.end.parse::<DateTime<Utc>>()) {
            let start = with_timezone(start).naive_local();
            let end = with_timezone(end).naive_local();
            match get_data_in_range(&pool, (start,end),db_name,&get_table_name_prefix()).await {
                Ok(res) => {
                    let res = analysis_history_data(res, 350, 1, 10);
                    HttpResponse::Ok().json(res)
                },
                Err(_e) => {return HttpResponse::InternalServerError().json("Error in sql") }
            }
        }else{
            HttpResponse::InternalServerError().json("Bad Time Range")
        }
    }else {
        HttpResponse::Unauthorized().json("Bad token")
    }
}


