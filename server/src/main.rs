use actix_web::{App, HttpServer, middleware::Logger};
use actix_web::{web};
use actix_cors::Cors;
use dotenvy::dotenv;
use std::io;

mod config;
mod controllers;
mod models;
mod routes;
mod services;
mod database;

#[actix_web::main]
async fn main() -> io::Result<()> {
    // Initialize environment
    dotenv().ok();
    env_logger::init();

    let database_url = config::get_database_url();
    let server_url = config::get_server_url();
    log::info!("Starting server at: {}", server_url);

    let pool = database::db_connection::create_connection_pool(&database_url);
    let mut conn = database::db_connection::get_connection(&pool)
        .expect("Failed to get connection from pool");
    database::db_migrations::run_migrations(&mut conn);

    HttpServer::new(move || {
        // Configure custom logger
        let logger = Logger::new("%a \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T");

        // Configure CORS
        let cors = Cors::default()
            .allowed_origin("http://localhost:4200")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "PATCH"])
            .allowed_headers(vec!["content-type", "authorization"])
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(cors)
            .wrap(logger)
            .app_data(config::errors::json_error_handler())
            .configure(routes::configure)
    })
    .bind(server_url)?
    .run()
    .await
}
