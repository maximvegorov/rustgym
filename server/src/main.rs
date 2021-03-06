#[macro_use]
extern crate diesel;
#[macro_use]
extern crate derive_new;

mod agents;
mod app_data;
mod client;
mod context;
mod db;
mod routes;
mod session_data;

use actix::prelude::*;
use actix_session::CookieSession;
use actix_web::middleware::Logger;
use actix_web::web;
use actix_web::App;
use actix_web::HttpServer;
use agents::registry::RegistryAgent;
use app_data::AppData;
use log::info;
use rustgym_consts::*;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    let pool = db::init_pool(DATABASE_URL).expect("Failed to create pool");
    let tag = env::var("TAG").unwrap_or_default();
    let title = "RUST GYM".to_string();
    let app_data = AppData::new(tag.clone(), title.clone());

    // Start chat server actor
    let registry_addr = RegistryAgent::new().start();
    info!("RUST GYM Server {}", tag);
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .data(app_data.clone())
            .data(pool.clone())
            .data(registry_addr.clone())
            .service(routes::home::home)
            .service(routes::leetcode_index::leetcode_index)
            .service(routes::adventofcode_index::adventofcode_index)
            .service(routes::leetcode_detail::leetcode_detail)
            .service(routes::adventofcode_detail::adventofcode_detail)
            .service(routes::robots::robots_txt)
            .service(routes::sitemap::sitemap_txt)
            .service(client::client_files)
            .service(web::resource("/ws/").to(agents::websocket::ws_index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
