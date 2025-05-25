use diesel::prelude::*;
use uuid::Uuid;

use crate::models::{User, NewUser, UpdateUser};
use crate::models::schema::users;


pub fn get_all_users(conn: &mut PgConnection) -> Result<Vec<User>, diesel::result::Error> {
    users::table.load::<User>(conn)
}

pub fn get_user_by_id(conn: &mut PgConnection, user_id: Uuid) -> Result<User, diesel::result::Error> {
    users::table.find(user_id).first::<User>(conn)
}

pub fn create_user(conn: &mut PgConnection, new_user: NewUser) -> Result<User, diesel::result::Error> {
    diesel::insert_into(users::table)
        .values(new_user)
        .get_result(conn)
}

pub fn update_user(conn: &mut PgConnection, user_id: Uuid, update_user: UpdateUser) -> Result<User, diesel::result::Error> {
    diesel::update(users::table.find(user_id))
}