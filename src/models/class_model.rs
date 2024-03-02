use chrono::NaiveDateTime;
use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::classes;

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
#[diesel(table_name = classes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ClassModel {
    pub id: Uuid,
    pub name: String,
    pub student_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

impl ClassModel {
    pub fn new(name: String, student_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            student_id,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: None,
        }
    }
}
