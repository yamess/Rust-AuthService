use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::schools;

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
#[diesel(table_name = schools)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SchoolModel {
    pub id: Uuid,
    pub name: String,
    pub website: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

impl SchoolModel {
    pub fn new(name: String, website: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            website,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: None,
        }
    }
}
