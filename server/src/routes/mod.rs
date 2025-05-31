mod user_routes;
mod income_routes;
mod expense_routes;

use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(user_routes::configure)
            .configure(income_routes::configure)
            .configure(expense_routes::configure)
    );
} 