use actix_web::{web, HttpResponse};
use chrono::{NaiveDate, Utc};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::config::errors::{AppError, response};
use crate::database::DbPool;
use crate::models::{NewExpense, UpdateExpense};
use crate::services::expense_service;

// Helper function to validate expense data
fn validate_expense_data(item_name: &str, amount: &str) -> Result<(), AppError> {
    if item_name.trim().is_empty() {
        return Err(AppError::Validation("Item name cannot be empty".to_string()));
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

/// Path parameters for expense endpoints
#[derive(IntoParams)]
pub struct ExpensePathParams {
    /// User ID
    userId: String,
}

/// Path parameters for specific expense endpoints
#[derive(IntoParams)]
pub struct ExpenseIdPathParams {
    /// User ID
    userId: String,
    /// Expense ID
    expenseId: String,
}

/// Get all expenses for a user
#[utoipa::path(
    get,
    path = "/api/users/{userId}/expenses",
    params(ExpensePathParams),
    responses(
        (status = 200, description = "List of expense records", body = [Expense]),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "expenses"
)]
pub async fn get_all_expenses(
    pool: web::Data<DbPool>,
    user_id: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let user_id = Uuid::parse_str(&user_id.into_inner())?;
    let mut conn = pool.get()?;
    let expenses = expense_service::get_all_expenses_for_user(&mut conn, user_id)?;
    Ok(response::ok(expenses))
}

/// Get a specific expense record by ID
#[utoipa::path(
    get,
    path = "/api/users/{userId}/expenses/{expenseId}",
    params(ExpenseIdPathParams),
    responses(
        (status = 200, description = "Expense record found", body = Expense),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Expense record not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "expenses"
)]
pub async fn get_expense_by_id(
    pool: web::Data<DbPool>,
    path: web::Path<(String, String)>,
) -> Result<HttpResponse, AppError> {
    let (user_id, expense_id) = path.into_inner();
    let user_id = Uuid::parse_str(&user_id)?;
    let expense_id = Uuid::parse_str(&expense_id)?;
    
    let mut conn = pool.get()?;
    let expense = expense_service::get_expense_by_id(&mut conn, expense_id, user_id)?;
    Ok(response::ok(expense))
}

// Struct for receiving expense creation data with string values for validation
#[derive(serde::Deserialize, ToSchema)]
pub struct CreateExpenseRequest {
    /// Item name or expense descriptor
    pub item_name: String,
    /// Expense amount as a string (will be validated and converted to decimal)
    pub amount: String,
    /// Date when expense occurred
    #[schema(value_type = String, example = "2023-01-01")]
    pub date: NaiveDate,
    /// Optional description of the expense
    pub description: Option<String>,
}

/// Create a new expense record
#[utoipa::path(
    post,
    path = "/api/users/{userId}/expenses",
    params(ExpensePathParams),
    request_body = CreateExpenseRequest,
    responses(
        (status = 201, description = "Expense record created", body = Expense),
        (status = 400, description = "Invalid input"),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "expenses"
)]
pub async fn create_expense(
    pool: web::Data<DbPool>,
    user_id: web::Path<String>,
    expense_data: web::Json<CreateExpenseRequest>,
) -> Result<HttpResponse, AppError> {
    // Validate expense data
    validate_expense_data(&expense_data.item_name, &expense_data.amount)?;
    
    let user_id = Uuid::parse_str(&user_id.into_inner())?;
    let amount = expense_data.amount.parse::<rust_decimal::Decimal>()
        .map_err(|_| AppError::Validation("Invalid amount format".to_string()))?;
    
    let new_expense = NewExpense {
        user_id,
        item_name: expense_data.item_name.clone(),
        amount: crate::models::decimal::PgDecimal(amount),
        date: expense_data.date,
        description: expense_data.description.clone(),
    };
    
    let mut conn = pool.get()?;
    let expense = expense_service::create_expense(&mut conn, new_expense)?;
    Ok(response::created(expense))
}

// Struct for receiving expense update data with string values for validation
#[derive(serde::Deserialize, ToSchema)]
pub struct UpdateExpenseRequest {
    /// Item name or expense descriptor (optional)
    pub item_name: Option<String>,
    /// Expense amount as a string (optional)
    pub amount: Option<String>,
    /// Date when expense occurred (optional)
    #[schema(value_type = Option<String>, example = "2023-01-01")]
    pub date: Option<NaiveDate>,
    /// Description of the expense (optional)
    pub description: Option<String>,
}

/// Update an existing expense record
#[utoipa::path(
    patch,
    path = "/api/users/{userId}/expenses/{expenseId}",
    params(ExpenseIdPathParams),
    request_body = UpdateExpenseRequest,
    responses(
        (status = 200, description = "Expense record updated", body = Expense),
        (status = 400, description = "Invalid input"),
        (status = 404, description = "Expense record not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "expenses"
)]
pub async fn update_expense(
    pool: web::Data<DbPool>,
    path: web::Path<(String, String)>,
    expense_data: web::Json<UpdateExpenseRequest>,
) -> Result<HttpResponse, AppError> {
    let (user_id, expense_id) = path.into_inner();
    let user_id = Uuid::parse_str(&user_id)?;
    let expense_id = Uuid::parse_str(&expense_id)?;
    
    // Validate if provided
    if let Some(item_name) = &expense_data.item_name {
        if item_name.trim().is_empty() {
            return Err(AppError::Validation("Item name cannot be empty".to_string()));
        }
    }
    
    // Parse and validate amount if provided
    let amount = if let Some(amount_str) = &expense_data.amount {
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
    
    let update_data = UpdateExpense {
        item_name: expense_data.item_name.clone(),
        amount,
        date: expense_data.date,
        description: expense_data.description.clone(),
        updated_at: Utc::now().naive_utc(),
    };
    
    let mut conn = pool.get()?;
    let expense = expense_service::update_expense(&mut conn, expense_id, user_id, update_data)?;
    Ok(response::ok(expense))
}

/// Delete an expense record
#[utoipa::path(
    delete,
    path = "/api/users/{userId}/expenses/{expenseId}",
    params(ExpenseIdPathParams),
    responses(
        (status = 200, description = "Expense record deleted successfully"),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Expense record not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "expenses"
)]
pub async fn delete_expense(
    pool: web::Data<DbPool>,
    path: web::Path<(String, String)>,
) -> Result<HttpResponse, AppError> {
    let (user_id, expense_id) = path.into_inner();
    let user_id = Uuid::parse_str(&user_id)?;
    let expense_id = Uuid::parse_str(&expense_id)?;
    
    let mut conn = pool.get()?;
    let count = expense_service::delete_expense(&mut conn, expense_id, user_id)?;
    
    if count > 0 {
        Ok(response::ok(serde_json::json!({
            "message": "Expense record deleted successfully"
        })))
    } else {
        Err(AppError::NotFound("Expense record not found".to_string()))
    }
} 