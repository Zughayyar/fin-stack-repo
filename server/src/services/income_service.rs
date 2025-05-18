use diesel::prelude::*;
use uuid::Uuid;

use crate::database::DbConnection;
use crate::models::{Income, NewIncome, UpdateIncome};
use crate::models::schema::income;

pub fn get_all_income_for_user(
    conn: &mut DbConnection,
    user_id: Uuid,
) -> Result<Vec<Income>, diesel::result::Error> {
    income::table
        .filter(income::user_id.eq(user_id))
        .order_by(income::date.desc())
        .select(Income::as_select())
        .load(conn)
}

pub fn get_income_by_id(
    conn: &mut DbConnection,
    income_id: Uuid,
    user_id: Uuid,
) -> Result<Income, diesel::result::Error> {
    income::table
        .find(income_id)
        .filter(income::user_id.eq(user_id))
        .select(Income::as_select())
        .first(conn)
}

pub fn create_income(
    conn: &mut DbConnection,
    new_income: NewIncome,
) -> Result<Income, diesel::result::Error> {
    diesel::insert_into(income::table)
        .values(new_income)
        .returning(Income::as_select())
        .get_result(conn)
}

pub fn update_income(
    conn: &mut DbConnection,
    income_id: Uuid,
    user_id: Uuid,
    update_data: UpdateIncome,
) -> Result<Income, diesel::result::Error> {
    diesel::update(
        income::table
            .find(income_id)
            .filter(income::user_id.eq(user_id)),
    )
    .set(update_data)
    .returning(Income::as_select())
    .get_result(conn)
}

pub fn delete_income(
    conn: &mut DbConnection,
    income_id: Uuid,
    user_id: Uuid,
) -> Result<usize, diesel::result::Error> {
    diesel::delete(
        income::table
            .find(income_id)
            .filter(income::user_id.eq(user_id)),
    )
    .execute(conn)
} 