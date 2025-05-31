use actix_web::{web, HttpResponse};
use diesel::PgConnection;
use r2d2::Pool;
use diesel::r2d2::ConnectionManager;
use uuid::Uuid;
use crate::models::expense::{NewExpense, UpdateExpense};

use crate::config::errors::{AppError, response};
use crate::services::expense_service;

type DbPool = Pool<ConnectionManager<PgConnection>>;

pub async fn get_all_expenses(pool: web::Data<DbPool>) -> Result<HttpResponse, AppError> {
    let mut conn = pool.get()?;
    let expenses = expense_service::get_all_expenses(&mut conn)?;
    Ok(response::ok(expenses))
}

pub async fn get_expenses_by_user_id(pool: web::Data<DbPool>, user_id: web::Path<Uuid>) -> Result<HttpResponse, AppError> {
    let mut conn = pool.get()?;
    let expenses = expense_service::get_expenses_by_user_id(&mut conn, user_id.into_inner())?;
    Ok(response::ok(expenses))
}

pub async fn create_expense(pool: web::Data<DbPool>, new_expense: web::Json<NewExpense>) -> Result<HttpResponse, AppError> {
    let mut conn = pool.get()?;
    let expense = expense_service::create_expense(&mut conn, new_expense.into_inner())?;
    Ok(response::created(expense))
}

pub async fn update_expense(pool: web::Data<DbPool>, expense_id: web::Path<Uuid>, update_expense: web::Json<UpdateExpense>) -> Result<HttpResponse, AppError> {
    let mut conn = pool.get()?;
    let expense = expense_service::update_expense(&mut conn, expense_id.into_inner(), update_expense.into_inner())?;
    let response = HttpResponse::Ok()
        .body(format!("Expense with id {} updated successfully", expense.id));
    Ok(response)
}

pub async fn delete_expense(pool: web::Data<DbPool>, expense_id: web::Path<Uuid>) -> Result<HttpResponse, AppError> {
    let mut conn = pool.get()?;
    let expense = expense_service::delete_expense(&mut conn, expense_id.into_inner())?;
    let response = HttpResponse::Ok()
        .body(format!("Expense with id {} deleted successfully", expense.id));
    Ok(response)
}
