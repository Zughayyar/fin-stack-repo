use actix_web::{App, HttpServer, middleware::Logger};
use actix_web::{web, HttpResponse, Responder};
use actix_cors::Cors;
use dotenvy::dotenv;
use std::io;

mod config;
mod controllers;
mod models;
mod routes;
mod services;


// Redirect handler for /swagger-ui to /swagger-ui/
async fn redirect_swagger() -> impl Responder {
    HttpResponse::Found()
        .append_header(("Location", "/swagger-ui/"))
        .finish()
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    // Initialize environment
    dotenv().ok();
    env_logger::init();

    let server_url = config::get_server_url();
    log::info!("Starting server at: {}", server_url);
    log::info!("Swagger UI available at: {}/swagger-ui/", server_url);

    HttpServer::new(move || {
        // Configure custom logger to exclude Swagger UI requests
        let logger = Logger::new("%a \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T")
            .exclude_regex("/swagger-ui/.*")
            .exclude_regex("/api-docs/.*")
            .exclude_regex("/.well-known/appspecific/.*");

        // Configure CORS
        let cors = Cors::default()
            .allowed_origin("http://localhost:4200")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "PATCH"])
            .allowed_headers(vec!["content-type", "authorization"])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(logger)
            .app_data(config::errors::json_error_handler())
            .route("/swagger-ui", web::get().to(redirect_swagger))
            .configure(routes::configure)
    })
    .bind(server_url)?
    .run()
    .await
}


