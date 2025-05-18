use utoipa::OpenApi;
use utoipa::openapi::schema::{Schema, SchemaType, ObjectBuilder};
use chrono::{NaiveDateTime, NaiveDate};
use uuid::Uuid;
use crate::models::{Income, NewIncome, UpdateIncome, Expense, NewExpense, UpdateExpense, User, NewUser, UpdateUser};
use crate::controllers::{income_controller, expense_controller, user_controller};

/// Custom schema for Uuid type
#[derive(Default, utoipa::ToSchema)]
pub struct UuidSchema(String);

/// Custom schema for DateTime type
#[derive(Default, utoipa::ToSchema)]
pub struct NaiveDateTimeSchema(String);

/// Custom schema for Date type
#[derive(Default, utoipa::ToSchema)]
pub struct NaiveDateSchema(String);

/// API documentation
#[derive(OpenApi)]
#[openapi(
    paths(
        // User endpoints
        user_controller::get_all_users,
        user_controller::get_user_by_id,
        user_controller::create_user,
        user_controller::update_user,
        user_controller::delete_user,
        // Income endpoints
        income_controller::get_all_income,
        income_controller::get_income_by_id,
        income_controller::create_income,
        income_controller::update_income,
        income_controller::delete_income,
        // Expense endpoints
        expense_controller::get_all_expenses,
        expense_controller::get_expense_by_id,
        expense_controller::create_expense,
        expense_controller::update_expense,
        expense_controller::delete_expense,
    ),
    components(
        schemas(
            User, NewUser, UpdateUser,
            Income, NewIncome, UpdateIncome, 
            Expense, NewExpense, UpdateExpense,
            income_controller::CreateIncomeRequest,
            income_controller::UpdateIncomeRequest,
            expense_controller::CreateExpenseRequest,
            expense_controller::UpdateExpenseRequest,
            UuidSchema, NaiveDateTimeSchema, NaiveDateSchema
        )
    ),
    tags(
        (name = "users", description = "User management endpoints"),
        (name = "income", description = "Income management endpoints"),
        (name = "expenses", description = "Expense management endpoints")
    ),
    info(
        title = "Financial Management API",
        version = "1.0.0",
        description = "API for tracking personal income and expenses"
    )
)]
pub struct ApiDoc; 