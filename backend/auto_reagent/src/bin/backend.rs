use actix_web::{get, guard, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_cors::Cors;
extern crate AutoReagent;
use AutoReagent::handlers::{login::*,monitor::*,machine_panel::*,history_data::*,};
use AutoReagent::middleware::{myws::{MyWs,websocket_index},redis_data::RedisState,sqlx_manager::SqlxManager};
use actix::prelude::Addr;
use std::sync::{RwLock,Arc};

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().unwrap();
    let db_names = ["flux","fluxVice","plc"];
    let mut sqlx_state = SqlxManager::new();
    for name in db_names { sqlx_state.add_database(name, dotenvy::var(name).unwrap()).await; }
    let redis_state= RedisState::new(dotenvy::var("REDIS_PASSWD").unwrap(), dotenvy::var("REDIS_URL").unwrap());
    let addr: Arc<RwLock<Vec<Addr<MyWs>>>> = Arc::new(RwLock::new(vec![]));
    HttpServer::new(move || {
       let cors = Cors::default()
            .allow_any_origin()
            //  .allow_any_header()
            //  .allow_any_method()
            //  .allowed_origin("http://localhost:5173")
            //  .allowed_origin("http://47.92.144.135")
            .supports_credentials()
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![actix_web::http::header::AUTHORIZATION, actix_web::http::header::ACCEPT,])
            .allowed_header(actix_web::http::header::CONTENT_TYPE)
            .allowed_header("token")
            .expose_headers(vec!["token"])
            .max_age(3600);
       App::new()
            .app_data(web::Data::new(sqlx_state.clone()))
            .app_data(web::Data::new(redis_state.clone()))
            .app_data(web::Data::new(addr.clone()))
            .wrap(cors)
            .service(findlast)
            .service(findlast_vice)
            .service(login)
            .service(turbine_state)
            .service(check_privilege)
            .service(main_history)
            .service(vice_history)
            .service(start_main)
            .service(stop_main)
            .service(start_vice)
            .service(stop_vice)
            .service(pump_status)
            .service(set_point)
            .service(websocket_index)
            // .service(send_instruction)
            // .default_service(web::to(|| HttpResponse::Ok()))
    })
    // .bind("0.0.0.0:8080")?
    .bind("localhost:8080")?
    .run()
    .await
}
