use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

use crate::schema::users;

#[derive(
    Insertable, Queryable, Selectable, Deserialize, Serialize, AsChangeset, Debug, PartialEq,
)]
#[diesel(table_name = users)]
pub struct UserModel {
    pub id: uuid::Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
}
