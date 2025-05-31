use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use rust_decimal::Decimal;
use crate::models::schema::incomes;
use diesel::{Queryable, Selectable, Insertable, AsChangeset};
use crate::models::user::User;

#[derive(Debug, Queryable, Selectable, Deserialize, Serialize)]
#[diesel(table_name = incomes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Income {
    pub id: Uuid,
    pub user_id: Uuid,
    pub source: String,
    #[serde(with = "rust_decimal::serde::float")]
    pub amount: Decimal,
    pub date: NaiveDate,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct IncomeWithUser {
    #[serde(flatten)]
    pub income: Income,
    pub user: User,
}

#[derive(Debug, Insertable, Deserialize, Serialize)]
#[diesel(table_name = incomes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewIncome {
    pub user_id: Uuid,
    pub source: String,
    #[serde(with = "rust_decimal::serde::float")]
    #[diesel(sql_type = diesel::sql_types::Numeric)]
    pub amount: Decimal,
    pub date: NaiveDate,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, AsChangeset)]
#[diesel(table_name = incomes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdateIncome {
    pub source: Option<String>,
    pub amount: Option<Decimal>,
    pub date: Option<NaiveDate>,
    pub description: Option<String>,
}