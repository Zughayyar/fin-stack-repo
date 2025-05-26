use actix_web::web;
use crate::controllers::income_controller;



pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/incomes")
            .route("", web::get().to(income_controller::get_all_incomes))
            .route("/{user_id}", web::get().to(income_controller::get_incomes_by_user_id))
            .route("", web::post().to(income_controller::create_income))
    );
} 