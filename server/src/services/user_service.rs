use diesel::prelude::*;
use uuid::Uuid;
use chrono::Utc;

use crate::models::{User, NewUser, UpdateUser};
use crate::models::schema::users;
use crate::database::db_connection::DbConnection;

pub fn get_all_users(connection: &mut DbConnection) -> Result<Vec<User>, diesel::result::Error> {
    users::table
        .load::<User>(connection)
}

pub fn get_user_by_id(connection: &mut DbConnection, user_id: Uuid) -> Result<User, diesel::result::Error> {
    users::table
        .filter(users::id.eq(user_id))
        .first::<User>(connection)
}

pub fn create_user(connection: &mut DbConnection, new_user: NewUser) -> Result<User, diesel::result::Error> {
    connection.transaction(|connection| {
        let user = new_user.into_user();
        diesel::insert_into(users::table)
            .values(&user)
            .get_result(connection)
    })
}

pub fn update_user(connection: &mut DbConnection, user_id: Uuid, mut update_user: UpdateUser) -> Result<User, diesel::result::Error> {
    connection.transaction(|connection| {
        update_user.updated_at = Some(Utc::now().naive_utc());
        diesel::update(users::table.find(user_id))
            .set(update_user)
            .get_result(connection)
    })
}

pub fn delete_user(connection: &mut DbConnection, user_id: Uuid) -> Result<User, diesel::result::Error> {
    connection.transaction(|connection| {
        diesel::delete(users::table.find(user_id))
            .get_result(connection)
    })
}