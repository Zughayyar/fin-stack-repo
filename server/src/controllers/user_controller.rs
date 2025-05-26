use actix_web::{web, HttpResponse};
use uuid::Uuid;
use diesel::PgConnection;
use r2d2::Pool;
use diesel::r2d2::ConnectionManager;

use crate::config::errors::{AppError, response};
use crate::models::{NewUser, UpdateUser};
use crate::services::user_service;

type DbPool = Pool<ConnectionManager<PgConnection>>;

pub async fn get_all_users(pool: web::Data<DbPool>) -> Result<HttpResponse, AppError> {
    let mut conn = pool.get()?;
    let users = user_service::get_all_users(&mut conn)?;
    Ok(response::ok(users))
}

pub async fn get_user_by_id(pool: web::Data<DbPool>, path: web::Path<Uuid>) -> Result<HttpResponse, AppError> {
    let mut conn = pool.get()?;
    let user = user_service::get_user_by_id(&mut conn, path.into_inner())?;
    Ok(response::ok(user))
}

pub async fn create_user(pool: web::Data<DbPool>, new_user: web::Json<NewUser>) -> Result<HttpResponse, AppError> {
    println!("Creating user: {:?}", new_user);
    let mut conn = pool.get()?;
    let user = user_service::create_user(&mut conn, new_user.into_inner())?;
    Ok(response::created(user))
}

pub async fn update_user(pool: web::Data<DbPool>, path: web::Path<Uuid>, update_user: web::Json<UpdateUser>) -> Result<HttpResponse, AppError> {
    let mut conn = pool.get()?;
    let user = user_service::update_user(&mut conn, path.into_inner(), update_user.into_inner())?;
    Ok(response::ok(user))
}

pub async fn delete_user(pool: web::Data<DbPool>, path: web::Path<Uuid>) -> Result<HttpResponse, AppError> {
    let mut conn = pool.get()?;
    let user = user_service::delete_user(&mut conn, path.into_inner())?;
    Ok(response::ok(user))
}