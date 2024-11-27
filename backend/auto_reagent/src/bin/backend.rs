use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
extern crate AutoReagent;
use AutoReagent::router::login::*;
use AutoReagent::middleware::{myws::{MyWs,websocket_index},redis_data::RedisState,sqlx_manager::SqlxManager};
use actix::prelude::Addr;
use std::sync::{RwLock,Arc};
use AutoReagent::utility::{parameter::Args, config::Config};
use clap::Parser;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let paras = Args::parse();
    let config_file = paras.config.as_ref();
    let config = Config::init(config_file.expect("No such file for config initialization"));

    let mut sqlx_state = SqlxManager::new();
    sqlx_state.add_databases(&config.database_url).await;
    let redis_state= RedisState::new(config.redis_password, config.redis_url);
    let addr: Arc<RwLock<Vec<Addr<MyWs>>>> = Arc::new(RwLock::new(vec![]));
    let port = config.port;
    let host_mode = config.host;
    println!("Application runs on {host_mode}:{port}");

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
            .service(login)
            .service(check_privilege)
            .service(websocket_index)
            .service(useradd)
          
            // .service(send_instruction)
            // .default_service(web::to(|| HttpResponse::Ok()))
    })
    .bind(format!("{host_mode}:{port}"))?
    .run()
    .await
}
