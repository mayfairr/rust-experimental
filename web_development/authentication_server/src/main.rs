use actix_web::{http, App, HttpServer};
use actix_cors::Cors;
use env_logger;
use dotenv;

mod api;
mod config;
mod services;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::from_filename(".env").expect("Failed to read .env file");

    std::env::set_var("RUST_LOG", "actix_web=info");
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let app_host = std::env::var("APP_HOST").expect("APP_HOST not found.");
    let app_port = std::env::var("APP_PORT").expect("APP_PORT not found.");
    let app_url = format!("{}:{}", &app_host, &app_port);

    HttpServer::new(|| {
        App::new().wrap(
            Cors::default() // allowed_origin return access-control-allow-origin: * by default
            .send_wildcard()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600))
            .wrap(actix_web::middleware::Logger::default())
            .configure(config::app::config_services)         
    })
    .bind(&app_url)?
    .run()
    .await
}

