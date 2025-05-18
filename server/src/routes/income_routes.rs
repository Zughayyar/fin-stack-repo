use actix_web::web;
use crate::controllers::income_controller;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/users/{userId}/income")
            .route("", web::get().to(income_controller::get_all_income))
            .route("", web::post().to(income_controller::create_income))
            .route("/{incomeId}", web::get().to(income_controller::get_income_by_id))
            .route("/{incomeId}", web::patch().to(income_controller::update_income))
            .route("/{incomeId}", web::delete().to(income_controller::delete_income)),
    );
} 