use actix_web::{web, HttpResponse};
use chrono::{NaiveDate, Utc};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::config::errors::{AppError, response};
use crate::database::DbPool;
use crate::models::{NewIncome, UpdateIncome};
use crate::services::income_service;

// Helper function to validate income data
fn validate_income_data(source: &str, amount: &str) -> Result<(), AppError> {
    if source.trim().is_empty() {
        return Err(AppError::Validation("Source cannot be empty".to_string()));
    }
    
    if amount.trim().is_empty() {
        return Err(AppError::Validation("Amount cannot be empty".to_string()));
    }
    
    // Validate amount is a valid decimal number
    match amount.parse::<rust_decimal::Decimal>() {
        Ok(decimal) => {
            if decimal <= rust_decimal::Decimal::ZERO {
                return Err(AppError::Validation("Amount must be greater than zero".to_string()));
            }
        },
        Err(_) => return Err(AppError::Validation("Invalid amount format".to_string())),
    }
    
    Ok(())
}

/// Path parameters for income endpoints
#[derive(IntoParams)]
pub struct IncomePathParams {
    /// User ID
    userId: String,
}

/// Path parameters for specific income endpoints
#[derive(IntoParams)]
pub struct IncomeIdPathParams {
    /// User ID
    userId: String,
    /// Income ID
    incomeId: String,
}

/// Get all income records for a user
#[utoipa::path(
    get,
    path = "/api/users/{userId}/income",
    params(IncomePathParams),
    responses(
        (status = 200, description = "List of income records", body = [Income]),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "income"
)]
pub async fn get_all_income(
    pool: web::Data<DbPool>,
    user_id: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let user_id = Uuid::parse_str(&user_id.into_inner())?;
    let mut conn = pool.get()?;
    let income = income_service::get_all_income_for_user(&mut conn, user_id)?;
    Ok(response::ok(income))
}

/// Get a specific income record by ID
#[utoipa::path(
    get,
    path = "/api/users/{userId}/income/{incomeId}",
    params(IncomeIdPathParams),
    responses(
        (status = 200, description = "Income record found", body = Income),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Income record not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "income"
)]
pub async fn get_income_by_id(
    pool: web::Data<DbPool>,
    path: web::Path<(String, String)>,
) -> Result<HttpResponse, AppError> {
    let (user_id, income_id) = path.into_inner();
    let user_id = Uuid::parse_str(&user_id)?;
    let income_id = Uuid::parse_str(&income_id)?;
    
    let mut conn = pool.get()?;
    let income = income_service::get_income_by_id(&mut conn, income_id, user_id)?;
    Ok(response::ok(income))
}

// Struct for receiving income creation data with string values for validation
#[derive(serde::Deserialize, ToSchema)]
pub struct CreateIncomeRequest {
    /// Source of the income (e.g., "Salary", "Freelance", etc.)
    pub source: String,
    /// Income amount as a string (will be validated and converted to decimal)
    pub amount: String,
    /// Date when income was received
    #[schema(value_type = String, example = "2023-01-01")]
    pub date: NaiveDate,
    /// Optional description of the income
    pub description: Option<String>,
}

/// Create a new income record
#[utoipa::path(
    post,
    path = "/api/users/{userId}/income",
    params(IncomePathParams),
    request_body = CreateIncomeRequest,
    responses(
        (status = 201, description = "Income record created", body = Income),
        (status = 400, description = "Invalid input"),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "income"
)]
pub async fn create_income(
    pool: web::Data<DbPool>,
    user_id: web::Path<String>,
    income_data: web::Json<CreateIncomeRequest>,
) -> Result<HttpResponse, AppError> {
    // Validate income data
    validate_income_data(&income_data.source, &income_data.amount)?;
    
    let user_id = Uuid::parse_str(&user_id.into_inner())?;
    let amount = income_data.amount.parse::<rust_decimal::Decimal>()
        .map_err(|_| AppError::Validation("Invalid amount format".to_string()))?;
    
    let new_income = NewIncome {
        user_id,
        source: income_data.source.clone(),
        amount: crate::models::decimal::PgDecimal(amount),
        date: income_data.date,
        description: income_data.description.clone(),
    };
    
    let mut conn = pool.get()?;
    let income = income_service::create_income(&mut conn, new_income)?;
    Ok(response::created(income))
}

// Struct for receiving income update data with string values for validation
#[derive(serde::Deserialize, ToSchema)]
pub struct UpdateIncomeRequest {
    /// Source of the income (optional)
    pub source: Option<String>,
    /// Income amount as a string (optional)
    pub amount: Option<String>,
    /// Date when income was received (optional)
    #[schema(value_type = Option<String>, example = "2023-01-01")]
    pub date: Option<NaiveDate>,
    /// Description of the income (optional)
    pub description: Option<String>,
}

/// Update an existing income record
#[utoipa::path(
    patch,
    path = "/api/users/{userId}/income/{incomeId}",
    params(IncomeIdPathParams),
    request_body = UpdateIncomeRequest,
    responses(
        (status = 200, description = "Income record updated", body = Income),
        (status = 400, description = "Invalid input"),
        (status = 404, description = "Income record not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "income"
)]
pub async fn update_income(
    pool: web::Data<DbPool>,
    path: web::Path<(String, String)>,
    income_data: web::Json<UpdateIncomeRequest>,
) -> Result<HttpResponse, AppError> {
    let (user_id, income_id) = path.into_inner();
    let user_id = Uuid::parse_str(&user_id)?;
    let income_id = Uuid::parse_str(&income_id)?;
    
    // Validate if provided
    if let Some(source) = &income_data.source {
        if source.trim().is_empty() {
            return Err(AppError::Validation("Source cannot be empty".to_string()));
        }
    }
    
    // Parse and validate amount if provided
    let amount = if let Some(amount_str) = &income_data.amount {
        if amount_str.trim().is_empty() {
            return Err(AppError::Validation("Amount cannot be empty if provided".to_string()));
        }
        
        match amount_str.parse::<rust_decimal::Decimal>() {
            Ok(decimal) => {
                if decimal <= rust_decimal::Decimal::ZERO {
                    return Err(AppError::Validation("Amount must be greater than zero".to_string()));
                }
                Some(crate::models::decimal::PgDecimal(decimal))
            },
            Err(_) => return Err(AppError::Validation("Invalid amount format".to_string())),
        }
    } else {
        None
    };
    
    let update_data = UpdateIncome {
        source: income_data.source.clone(),
        amount,
        date: income_data.date,
        description: income_data.description.clone(),
        updated_at: Utc::now().naive_utc(),
    };
    
    let mut conn = pool.get()?;
    let income = income_service::update_income(&mut conn, income_id, user_id, update_data)?;
    Ok(response::ok(income))
}

/// Delete an income record
#[utoipa::path(
    delete,
    path = "/api/users/{userId}/income/{incomeId}",
    params(IncomeIdPathParams),
    responses(
        (status = 200, description = "Income record deleted successfully"),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Income record not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "income"
)]
pub async fn delete_income(
    pool: web::Data<DbPool>,
    path: web::Path<(String, String)>,
) -> Result<HttpResponse, AppError> {
    let (user_id, income_id) = path.into_inner();
    let user_id = Uuid::parse_str(&user_id)?;
    let income_id = Uuid::parse_str(&income_id)?;
    
    let mut conn = pool.get()?;
    let count = income_service::delete_income(&mut conn, income_id, user_id)?;
    
    if count > 0 {
        Ok(response::ok(serde_json::json!({
            "message": "Income record deleted successfully"
        })))
    } else {
        Err(AppError::NotFound("Income record not found".to_string()))
    }
} 