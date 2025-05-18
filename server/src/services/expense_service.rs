use diesel::prelude::*;
use uuid::Uuid;

use crate::database::DbConnection;
use crate::models::{Expense, NewExpense, UpdateExpense};
use crate::models::schema::expenses;

pub fn get_all_expenses_for_user(
    conn: &mut DbConnection,
    user_id: Uuid,
) -> Result<Vec<Expense>, diesel::result::Error> {
    expenses::table
        .filter(expenses::user_id.eq(user_id))
        .order_by(expenses::date.desc())
        .select(Expense::as_select())
        .load(conn)
}

pub fn get_expense_by_id(
    conn: &mut DbConnection,
    expense_id: Uuid,
    user_id: Uuid,
) -> Result<Expense, diesel::result::Error> {
    expenses::table
        .find(expense_id)
        .filter(expenses::user_id.eq(user_id))
        .select(Expense::as_select())
        .first(conn)
}

pub fn create_expense(
    conn: &mut DbConnection,
    new_expense: NewExpense,
) -> Result<Expense, diesel::result::Error> {
    diesel::insert_into(expenses::table)
        .values(new_expense)
        .returning(Expense::as_select())
        .get_result(conn)
}

pub fn update_expense(
    conn: &mut DbConnection,
    expense_id: Uuid,
    user_id: Uuid,
    update_data: UpdateExpense,
) -> Result<Expense, diesel::result::Error> {
    diesel::update(
        expenses::table
            .find(expense_id)
            .filter(expenses::user_id.eq(user_id)),
    )
    .set(update_data)
    .returning(Expense::as_select())
    .get_result(conn)
}

pub fn delete_expense(
    conn: &mut DbConnection,
    expense_id: Uuid,
    user_id: Uuid,
) -> Result<usize, diesel::result::Error> {
    diesel::delete(
        expenses::table
            .find(expense_id)
            .filter(expenses::user_id.eq(user_id)),
    )
    .execute(conn)
} 