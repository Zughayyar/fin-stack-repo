use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;
use crate::models::schema::users;
use crate::models::income::Income;

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Insertable, ToSchema)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub id: Uuid,
    #[schema(example = "John")]
    pub first_name: String,
    #[schema(example = "Doe")]
    pub last_name: String,
    #[schema(example = "john@example.com")]
    pub email: String,
    #[schema(example = "hashed_password_here")]
    pub password: String,
    #[schema(example = "2024-03-20T10:00:00")]
    pub created_at: NaiveDateTime,
    #[schema(example = "2024-03-20T10:00:00")]
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UserWithIncomes {
    #[serde(flatten)]
    pub user: User,
    pub incomes: Vec<Income>,
}

#[derive(Debug, Serialize, Deserialize, Insertable, ToSchema)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser {
    #[schema(example = "John")]
    pub first_name: String,
    #[schema(example = "Doe")]
    pub last_name: String,
    #[schema(example = "john@example.com")]
    pub email: String,
    #[schema(example = "password123")]
    pub password: String,
}

impl NewUser {
    pub fn into_user(self) -> User {
        let now = chrono::Utc::now().naive_utc();
        User {
            id: Uuid::new_v4(),
            first_name: self.first_name,
            last_name: self.last_name,
            email: self.email,
            password: self.password,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, ToSchema)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdateUser {
    #[schema(example = "John")]
    pub first_name: Option<String>,
    #[schema(example = "Doe")]
    pub last_name: Option<String>,
    #[schema(example = "john@example.com")]
    pub email: Option<String>,
    #[schema(example = "newpassword123")]
    pub password: Option<String>,
    pub updated_at: Option<NaiveDateTime>,
}