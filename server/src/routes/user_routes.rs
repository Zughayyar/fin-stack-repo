use actix_web::web;
use crate::controllers::user_controller;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/users")
            .route("", web::get().to(user_controller::get_all_users))
            .route("", web::post().to(user_controller::create_user))
            .route("/{userId}", web::get().to(user_controller::get_user_by_id))
            .route("/{userId}", web::patch().to(user_controller::update_user))
            .route("/{userId}", web::delete().to(user_controller::delete_user)),
    );
} 