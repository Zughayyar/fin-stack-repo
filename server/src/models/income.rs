use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::models::decimal::PgDecimal;
use crate::models::schema::income;
use crate::models::user::User;

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable, Associations, ToSchema)]
#[diesel(belongs_to(User))]
#[diesel(table_name = income)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Income {
    /// Unique identifier for the income record
    #[schema(value_type = String, example = "123e4567-e89b-12d3-a456-426614174000")]
    pub id: Uuid,
    /// User ID who owns this income record
    #[schema(value_type = String, example = "123e4567-e89b-12d3-a456-426614174000")]
    pub user_id: Uuid,
    /// Source of the income (e.g., "Salary", "Freelance", etc.)
    pub source: String,
    /// Amount of income
    #[schema(value_type = String, example = "1500.00")]
    pub amount: PgDecimal,
    /// Date when income was received
    #[schema(value_type = String, example = "2023-01-01")]
    pub date: NaiveDate,
    /// Optional description of the income
    pub description: Option<String>,
    /// When the record was created
    #[schema(value_type = String, example = "2023-01-01T00:00:00")]
    pub created_at: NaiveDateTime,
    /// When the record was last updated
    #[schema(value_type = String, example = "2023-01-01T00:00:00")]
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Insertable, ToSchema)]
#[diesel(table_name = income)]
pub struct NewIncome {
    /// User ID who owns this income record
    #[schema(value_type = String, example = "123e4567-e89b-12d3-a456-426614174000")]
    pub user_id: Uuid,
    /// Source of the income (e.g., "Salary", "Freelance", etc.)
    pub source: String,
    /// Amount of income
    #[schema(value_type = String, example = "1500.00")]
    pub amount: PgDecimal,
    /// Date when income was received
    #[schema(value_type = String, example = "2023-01-01")]
    pub date: NaiveDate,
    /// Optional description of the income
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, ToSchema)]
#[diesel(table_name = income)]
pub struct UpdateIncome {
    /// Source of the income (optional)
    pub source: Option<String>,
    /// Amount of income (optional)
    #[schema(value_type = Option<String>, example = "1500.00")]
    pub amount: Option<PgDecimal>,
    /// Date when income was received (optional)
    #[schema(value_type = Option<String>, example = "2023-01-01")]
    pub date: Option<NaiveDate>,
    /// Description of the income (optional)
    pub description: Option<String>,
    /// When the record was updated
    #[schema(value_type = String, example = "2023-01-01T00:00:00")]
    pub updated_at: NaiveDateTime,
} 