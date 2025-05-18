use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::models::schema::users;

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, ToSchema)]
#[diesel(table_name = users)]
pub struct User {
    /// Unique identifier for the user
    #[schema(value_type = String, example = "123e4567-e89b-12d3-a456-426614174000")]
    pub id: Uuid,
    /// User's first name
    pub first_name: String,
    /// User's last name
    pub last_name: String,
    /// User's email address
    pub email: String,
    /// User's password (hashed)
    #[schema(value_type = String, example = "********")]
    pub password: String,
    /// When the user account was created
    #[schema(value_type = String, example = "2023-01-01T00:00:00")]
    pub created_at: NaiveDateTime,
    /// When the user account was last updated
    #[schema(value_type = String, example = "2023-01-01T00:00:00")]
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Insertable, ToSchema)]
#[diesel(table_name = users)]
pub struct NewUser {
    /// User's first name
    pub first_name: String,
    /// User's last name
    pub last_name: String,
    /// User's email address
    pub email: String,
    /// User's password
    #[schema(value_type = String, example = "password123")]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, ToSchema)]
#[diesel(table_name = users)]
pub struct UpdateUser {
    /// User's first name (optional)
    pub first_name: Option<String>,
    /// User's last name (optional)
    pub last_name: Option<String>,
    /// User's email address (optional)
    pub email: Option<String>,
    /// User's password (optional)
    #[schema(value_type = Option<String>, example = "newpassword123")]
    pub password: Option<String>,
    /// When the user account was updated
    #[schema(value_type = String, example = "2023-01-01T00:00:00")]
    pub updated_at: NaiveDateTime,
} 