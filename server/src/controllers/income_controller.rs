use actix_web::{web, HttpResponse};
use diesel::PgConnection;
use r2d2::Pool;
use diesel::r2d2::ConnectionManager;
use uuid::Uuid;
use crate::models::income::{NewIncome, UpdateIncome};

use crate::config::errors::{AppError, response};
use crate::services::income_service;


type DbPool = Pool<ConnectionManager<PgConnection>>;


pub async fn get_all_incomes(pool: web::Data<DbPool>) -> Result<HttpResponse, AppError> {
    let mut conn = pool.get()?;
    let incomes = income_service::get_all_incomes(&mut conn)?;
    Ok(response::ok(incomes))
}

pub async fn get_incomes_by_user_id(pool: web::Data<DbPool>, user_id: web::Path<Uuid>) -> Result<HttpResponse, AppError> {
    let mut conn = pool.get()?;
    let incomes = income_service::get_incomes_by_user_id(&mut conn, user_id.into_inner())?;
    Ok(response::ok(incomes))
}

pub async fn create_income(pool: web::Data<DbPool>, new_income: web::Json<NewIncome>) -> Result<HttpResponse, AppError> {
    let mut conn = pool.get()?;
    let income = income_service::create_income(&mut conn, new_income.into_inner())?;
    Ok(response::created(income))
}

pub async fn update_income(pool: web::Data<DbPool>, income_id: web::Path<Uuid>, update_income: web::Json<UpdateIncome>) -> Result<HttpResponse, AppError> {
    let mut conn = pool.get()?;
    let income = income_service::update_income(&mut conn, income_id.into_inner(), update_income.into_inner())?;
    let response = HttpResponse::Ok()
        .body(format!("Income with id {} updated successfully", income.id));
    Ok(response)
}

pub async fn delete_income(pool: web::Data<DbPool>, income_id: web::Path<Uuid>) -> Result<HttpResponse, AppError> {
    let mut conn = pool.get()?;
    let income = income_service::delete_income(&mut conn, income_id.into_inner())?;
    let response = HttpResponse::Ok()
        .body(format!("Income with id {} deleted successfully", income.id));
    Ok(response)
}