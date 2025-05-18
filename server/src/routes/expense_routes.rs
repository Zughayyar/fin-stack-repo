use actix_web::web;
use crate::controllers::expense_controller;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/users/{userId}/expenses")
            .route("", web::get().to(expense_controller::get_all_expenses))
            .route("", web::post().to(expense_controller::create_expense))
            .route("/{expenseId}", web::get().to(expense_controller::get_expense_by_id))
            .route("/{expenseId}", web::patch().to(expense_controller::update_expense))
            .route("/{expenseId}", web::delete().to(expense_controller::delete_expense)),
    );
} 