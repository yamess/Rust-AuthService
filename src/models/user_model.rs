use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::users;

#[derive(
Insertable,
Queryable,
Identifiable,
Selectable,
Deserialize,
Serialize,
AsChangeset,
Debug,
PartialEq,
)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserModel {
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub is_active: bool,
    pub is_admin: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

impl UserModel {
    pub fn new(
        email: String,
        password: String,
        is_active: bool,
        is_admin: bool,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            email,
            password,
            is_active,
            is_admin,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: None,
        }
    }
}