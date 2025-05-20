pub mod user_routes;
pub mod income_routes;
pub mod expense_routes;

use actix_web::web;
 
pub fn configure(cfg: &mut web::ServiceConfig) {
    user_routes::configure(cfg);
    income_routes::configure(cfg);
    expense_routes::configure(cfg);
} 