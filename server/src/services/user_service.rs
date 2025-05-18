use diesel::prelude::*;
use uuid::Uuid;

use crate::database::DbConnection;
use crate::models::{NewUser, UpdateUser, User};
use crate::models::schema::users;

pub fn get_all_users(conn: &mut DbConnection) -> Result<Vec<User>, diesel::result::Error> {
    users::table.load::<User>(conn)
}

pub fn get_user_by_id(conn: &mut DbConnection, user_id: Uuid) -> Result<User, diesel::result::Error> {
    users::table.find(user_id).first::<User>(conn)
}

pub fn create_user(conn: &mut DbConnection, new_user: NewUser) -> Result<User, diesel::result::Error> {
    diesel::insert_into(users::table)
        .values(new_user)
        .get_result(conn)
}

pub fn update_user(conn: &mut DbConnection, user_id: Uuid, update_data: UpdateUser) -> Result<User, diesel::result::Error> {
    diesel::update(users::table.find(user_id))
        .set(update_data)
        .get_result(conn)
}

pub fn delete_user(conn: &mut DbConnection, user_id: Uuid) -> Result<usize, diesel::result::Error> {
    diesel::delete(users::table.find(user_id))
        .execute(conn)
} 