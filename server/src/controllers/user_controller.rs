use actix_web::{web, HttpResponse};
use chrono::Utc;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;
use serde_json::json;

use crate::config::errors::{AppError, response};
use crate::database::DbPool;
use crate::models::{NewUser, UpdateUser};
use crate::services::user_service;

// Helper function to validate user data
fn validate_new_user(user: &NewUser) -> Result<(), AppError> {
    if user.first_name.trim().is_empty() {
        return Err(AppError::Validation("First name cannot be empty".to_string()));
    }
    
    if user.last_name.trim().is_empty() {
        return Err(AppError::Validation("Last name cannot be empty".to_string()));
    }
    
    // Simple email validation
    if !user.email.contains('@') || !user.email.contains('.') {
        return Err(AppError::Validation("Invalid email format".to_string()));
    }
    
    if user.password.len() < 6 {
        return Err(AppError::Validation("Password must be at least 6 characters long".to_string()));
    }
    
    Ok(())
}

/// Path parameters for user endpoints
#[derive(IntoParams)]
pub struct UserIdParam {
    /// User ID
    userId: String,
}

/// Get all users
#[utoipa::path(
    get,
    path = "/api/users",
    responses(
        (status = 200, description = "List of all users", body = [User]),
        (status = 500, description = "Internal server error")
    ),
    tag = "users"
)]
pub async fn get_all_users(pool: web::Data<DbPool>) -> Result<HttpResponse, AppError> {
    let mut conn = pool.get()?;
    let users = user_service::get_all_users(&mut conn)?;
    Ok(response::ok(users))
}

/// Get a specific user by ID
#[utoipa::path(
    get,
    path = "/api/users/{userId}",
    params(UserIdParam),
    responses(
        (status = 200, description = "User found", body = User),
        (status = 400, description = "Invalid UUID format"),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "users"
)]
pub async fn get_user_by_id(
    pool: web::Data<DbPool>, 
    path: web::Path<String>
) -> Result<HttpResponse, AppError> {
    let user_id = Uuid::parse_str(&path.into_inner())?;
    let mut conn = pool.get()?;
    let user = user_service::get_user_by_id(&mut conn, user_id)?;
    Ok(response::ok(user))
}

/// Create a new user
#[utoipa::path(
    post,
    path = "/api/users",
    request_body = NewUser,
    responses(
        (status = 201, description = "User created successfully", body = User),
        (status = 400, description = "Invalid input data"),
        (status = 500, description = "Internal server error")
    ),
    tag = "users"
)]
pub async fn create_user(
    pool: web::Data<DbPool>,
    new_user: web::Json<NewUser>,
) -> Result<HttpResponse, AppError> {
    validate_new_user(&new_user)?;
    
    let mut conn = pool.get()?;
    let user = user_service::create_user(&mut conn, new_user.into_inner())?;
    Ok(response::created(user))
}

/// Update an existing user
#[utoipa::path(
    patch,
    path = "/api/users/{userId}",
    params(UserIdParam),
    request_body = UpdateUser,
    responses(
        (status = 200, description = "User updated successfully", body = User),
        (status = 400, description = "Invalid input data or UUID format"),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "users"
)]
pub async fn update_user(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
    user_data: web::Json<UpdateUser>,
) -> Result<HttpResponse, AppError> {
    let user_id = Uuid::parse_str(&path.into_inner())?;
    
    let mut update_data = user_data.into_inner();
    // Set updated_at to current time
    update_data.updated_at = Utc::now().naive_utc();

    let mut conn = pool.get()?;
    let user = user_service::update_user(&mut conn, user_id, update_data)?;
    Ok(response::ok(user))
}

/// Delete a user
#[utoipa::path(
    delete,
    path = "/api/users/{userId}",
    params(UserIdParam),
    responses(
        (status = 200, description = "User deleted successfully"),
        (status = 400, description = "Invalid UUID format"),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "users"
)]
pub async fn delete_user(
    pool: web::Data<DbPool>, 
    path: web::Path<String>
) -> Result<HttpResponse, AppError> {
    let user_id = Uuid::parse_str(&path.into_inner())?;
    
    let mut conn = pool.get()?;
    let count = user_service::delete_user(&mut conn, user_id)?;
    
    if count > 0 {
        Ok(response::ok(json!({
            "message": "User deleted successfully"
        })))
    } else {
        Err(AppError::NotFound("User not found".to_string()))
    }
} 