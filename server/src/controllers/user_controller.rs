use actix_web::{web, HttpResponse};
use uuid::Uuid;
use diesel::PgConnection;
use r2d2::Pool;
use diesel::r2d2::ConnectionManager;

use crate::config::errors::{AppError, response};
use crate::models::{NewUser, UpdateUser, User, UserWithIncomes};
use crate::services::user_service;

type DbPool = Pool<ConnectionManager<PgConnection>>;

/// Get all users
#[utoipa::path(
    get,
    path = "/api/users",
    responses(
        (status = 200, description = "List of users", body = Vec<User>),
        (status = 500, description = "Internal server error")
    ),
    tag = "users"
)]
pub async fn get_all_users(pool: web::Data<DbPool>) -> Result<HttpResponse, AppError> {
    let mut conn = pool.get()?;
    let users = user_service::get_all_users(&mut conn)?;
    Ok(response::ok(users))
}

/// Get user by ID
#[utoipa::path(
    get,
    path = "/api/users/{user_id}",
    responses(
        (status = 200, description = "User found", body = UserWithIncomes),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    ),
    params(
        ("user_id" = Uuid, Path, description = "User ID")
    ),
    tag = "users"
)]
pub async fn get_user_by_id(pool: web::Data<DbPool>, user_id: web::Path<Uuid>) -> Result<HttpResponse, AppError> {
    let mut conn = pool.get()?;
    let user = user_service::get_user_by_id(&mut conn, user_id.into_inner())?;
    Ok(response::ok(user))
}

/// Create new user
#[utoipa::path(
    post,
    path = "/api/users",
    request_body = NewUser,
    responses(
        (status = 201, description = "User created successfully", body = User),
        (status = 400, description = "Invalid input"),
        (status = 500, description = "Internal server error")
    ),
    tag = "users"
)]
pub async fn create_user(pool: web::Data<DbPool>, new_user: web::Json<NewUser>) -> Result<HttpResponse, AppError> {
    println!("Creating user: {:?}", new_user);
    let mut conn = pool.get()?;
    let user = user_service::create_user(&mut conn, new_user.into_inner())?;
    Ok(response::created(user))
}

/// Update user
#[utoipa::path(
    patch,
    path = "/api/users/{user_id}",
    request_body = UpdateUser,
    responses(
        (status = 200, description = "User updated successfully", body = User),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    ),
    params(
        ("user_id" = Uuid, Path, description = "User ID")
    ),
    tag = "users"
)]
pub async fn update_user(pool: web::Data<DbPool>, user_id: web::Path<Uuid>, update_user: web::Json<UpdateUser>) -> Result<HttpResponse, AppError> {
    let mut conn = pool.get()?;
    let user = user_service::update_user(&mut conn, user_id.into_inner(), update_user.into_inner())?;
    let response = HttpResponse::Ok()
        .body(format!("User with id {} updated successfully", user.id));
    Ok(response)
}

/// Delete user
#[utoipa::path(
    delete,
    path = "/api/users/{user_id}",
    responses(
        (status = 200, description = "User deleted successfully"),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    ),
    params(
        ("user_id" = Uuid, Path, description = "User ID")
    ),
    tag = "users"
)]
pub async fn delete_user(pool: web::Data<DbPool>, user_id: web::Path<Uuid>) -> Result<HttpResponse, AppError> {
    let mut conn = pool.get()?;
    let user = user_service::delete_user(&mut conn, user_id.into_inner())?;

    let response = HttpResponse::Ok()
        .body(format!("User with id {} deleted successfully", user.id));
    Ok(response)
}