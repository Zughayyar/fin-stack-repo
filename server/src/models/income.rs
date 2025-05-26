use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use rust_decimal::Decimal;
use crate::models::schema::incomes;

#[derive(Debug, Queryable, Selectable, Deserialize, Serialize, ToSchema)]
#[diesel(table_name = incomes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Income {
    #[schema(value_type = String, example = "123e4567-e89b-12d3-a456-426614174000")]
    pub id: Uuid,

    #[schema(value_type = String, example = "123e4567-e89b-12d3-a456-426614174000")]
    pub user_id: Uuid,

    #[schema(value_type = String, example = "Salary")]
    pub source: String,

    #[serde(with = "rust_decimal::serde::float")]
    #[schema(value_type = Number, example = 1000.00)]
    pub amount: Decimal,

    #[schema(value_type = String, example = "2021-01-01")]
    pub date: NaiveDate,

    #[schema(value_type = Option<String>, example = "Description of the income")]
    pub description: Option<String>,

    #[schema(value_type = String, example = "2021-01-01T00:00:00")]
    pub created_at: NaiveDateTime,
    
    #[schema(value_type = String, example = "2021-01-01T00:00:00")]
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable, Deserialize, Serialize, ToSchema)]
#[diesel(table_name = incomes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewIncome {
    #[schema(value_type = String, example = "123e4567-e89b-12d3-a456-426614174000")]
    pub user_id: Uuid,
    #[schema(value_type = String, example = "Salary")]
    pub source: String,
    #[serde(with = "rust_decimal::serde::float")]
    #[diesel(sql_type = diesel::sql_types::Numeric)]
    #[schema(value_type = Number, example = 1000.00)]
    pub amount: Decimal,
    #[schema(value_type = String, example = "2021-01-01")]
    pub date: NaiveDate,
    #[schema(value_type = Option<String>, example = "Description of the income")]
    pub description: Option<String>,
}