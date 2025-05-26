use actix_web::web;
mod user_routes;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(user_routes::configure)
    );
} 