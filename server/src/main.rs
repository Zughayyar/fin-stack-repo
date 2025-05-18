use actix_web::{App, HttpServer, middleware::Logger};
use actix_web::{web, HttpResponse, Responder, HttpRequest};
use dotenv::dotenv;
use std::io;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod config;
mod controllers;
mod database;
mod models;
mod routes;
mod services;
mod docs;

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
    log::info!("Swagger UI available at: {}/swagger-ui/", server_url);

    // Start HTTP server
    HttpServer::new(move || {
        // Configure custom logger to exclude Swagger UI requests
        let logger = Logger::new("%a \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T")
            .exclude_regex("/swagger-ui/.*")
            .exclude_regex("/api-docs/.*")
            .exclude_regex("/.well-known/appspecific/.*");

        App::new()
            .wrap(logger)
            .app_data(actix_web::web::Data::new(pool.clone()))
            .app_data(config::errors::json_error_handler())
            // Add redirect for /swagger-ui to /swagger-ui/
            .route("/swagger-ui", web::get().to(redirect_swagger))
            // Swagger UI configuration
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", docs::ApiDoc::openapi())
            )
            .configure(routes::configure)
    })
    .bind(server_url)?
    .run()
    .await
}
