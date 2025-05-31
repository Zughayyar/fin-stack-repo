use actix_web::web;
use crate::controllers::expense_controller;


pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/expenses")
            .route("", web::get().to(expense_controller::get_all_expenses))
            .route("", web::post().to(expense_controller::create_expense))
            .route("/{user_id}", web::get().to(expense_controller::get_expenses_by_user_id))
            .route("/{expense_id}", web::put().to(expense_controller::update_expense))
            .route("/{expense_id}", web::delete().to(expense_controller::delete_expense))
    );
}
