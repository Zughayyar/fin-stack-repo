use actix_web::{App, HttpServer, middleware::Logger};
use dotenv::dotenv;
use std::io;

mod config;
mod controllers;
mod database;
mod models;
mod routes;
mod services;

#[actix_web::main]
async fn main() -> io::Result<()> {
    // Initialize environment
    dotenv().ok();
    env_logger::init();

    // Create db connection pool
    let pool = database::establish_connection_pool();
    
    // Run migrations
    {
        let mut conn = pool.get().expect("Failed to get db connection from pool");
        database::run_migrations(&mut conn);
    }

    // Server configuration
    let server_url = config::get_server_url();
    log::info!("Starting server at: {}", server_url);

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(actix_web::web::Data::new(pool.clone()))
            .app_data(config::errors::json_error_handler())
            .configure(routes::configure)
    })
    .bind(server_url)?
    .run()
    .await
}
