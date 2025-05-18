use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::models::decimal::PgDecimal;
use crate::models::schema::expenses;
use crate::models::user::User;

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable, Associations, ToSchema)]
#[diesel(belongs_to(User))]
#[diesel(table_name = expenses)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Expense {
    /// Unique identifier for the expense record
    #[schema(value_type = String, example = "123e4567-e89b-12d3-a456-426614174000")]
    pub id: Uuid,
    /// User ID who owns this expense record
    #[schema(value_type = String, example = "123e4567-e89b-12d3-a456-426614174000")]
    pub user_id: Uuid,
    /// Name or description of the expense item
    pub item_name: String,
    /// Amount of the expense
    #[schema(value_type = String, example = "45.99")]
    pub amount: PgDecimal,
    /// Date when expense occurred
    #[schema(value_type = String, example = "2023-01-01")]
    pub date: NaiveDate,
    /// Optional additional description
    pub description: Option<String>,
    /// When the record was created
    #[schema(value_type = String, example = "2023-01-01T00:00:00")]
    pub created_at: NaiveDateTime,
    /// When the record was last updated
    #[schema(value_type = String, example = "2023-01-01T00:00:00")]
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Insertable, ToSchema)]
#[diesel(table_name = expenses)]
pub struct NewExpense {
    /// User ID who owns this expense record
    #[schema(value_type = String, example = "123e4567-e89b-12d3-a456-426614174000")]
    pub user_id: Uuid,
    /// Name or description of the expense item
    pub item_name: String,
    /// Amount of the expense
    #[schema(value_type = String, example = "45.99")]
    pub amount: PgDecimal,
    /// Date when expense occurred
    #[schema(value_type = String, example = "2023-01-01")]
    pub date: NaiveDate,
    /// Optional additional description
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, ToSchema)]
#[diesel(table_name = expenses)]
pub struct UpdateExpense {
    /// Name or description of the expense item (optional)
    pub item_name: Option<String>,
    /// Amount of the expense (optional)
    #[schema(value_type = Option<String>, example = "45.99")]
    pub amount: Option<PgDecimal>,
    /// Date when expense occurred (optional)
    #[schema(value_type = Option<String>, example = "2023-01-01")]
    pub date: Option<NaiveDate>,
    /// Additional description (optional)
    pub description: Option<String>,
    /// When the record was updated
    #[schema(value_type = String, example = "2023-01-01T00:00:00")]
    pub updated_at: NaiveDateTime,
} 