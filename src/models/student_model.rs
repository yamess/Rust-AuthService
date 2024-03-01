use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

use crate::schema::students;

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
#[diesel(table_name = students)]
pub struct StudentModel {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub program: String,
    pub department: Option<String>,
    pub user_id: uuid::Uuid,
    pub school_id: uuid::Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

impl StudentModel {
    pub fn new(
        first_name: String,
        last_name: String,
        program: String,
        department: Option<String>,
        user_id: uuid::Uuid,
        school_id: uuid::Uuid,
        created_at: chrono::NaiveDateTime,
        updated_at: Option<chrono::NaiveDateTime>,
    ) -> Self {
        Self {
            id: 0,
            first_name,
            last_name,
            program,
            department,
            user_id,
            school_id,
            created_at,
            updated_at,
        }
    }
}
