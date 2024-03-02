use chrono::NaiveDateTime;
use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
#[diesel(table_name = schedules)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ClassModel {
    pub id: Uuid,
    pub name: String,
    pub student_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}