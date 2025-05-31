use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use rust_decimal::Decimal;
use crate::models::schema::expenses;

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Insertable)]
#[diesel(table_name = expenses)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Expense {
    pub id: Uuid,
    pub user_id: Uuid,
    pub item_name: String,
    pub amount: Decimal,
    pub date: chrono::NaiveDate,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = expenses)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewExpense {
    pub user_id: Uuid,
    pub item_name: String,
    pub amount: Decimal,
    pub description: Option<String>,
}

impl NewExpense {
    pub fn into_expense(self) -> Expense {
        let now = chrono::Utc::now().naive_utc();
        Expense {
            id: Uuid::new_v4(),
            user_id: self.user_id,
            item_name: self.item_name,
            amount: self.amount,
            date: now.date(),
            description: self.description,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = expenses)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdateExpense {
    pub item_name: Option<String>,
    pub amount: Option<Decimal>,
    pub date: Option<chrono::NaiveDate>,
    pub description: Option<String>,
    pub updated_at: Option<NaiveDateTime>,
} 