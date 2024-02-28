// use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
// use serde::{Deserialize, Serialize};
//
// use crate::schema::students;
//
// #[derive(
//     Insertable,
//     Queryable,
//     Identifiable,
//     Selectable,
//     Deserialize,
//     Serialize,
//     AsChangeset,
//     Debug,
//     PartialEq,
// )]
// #[diesel(table_name = students)]
// pub struct StudentModel {
//     pub id: uuid::Uuid,
//     pub first_name: String,
//     pub last_name: String,
//     pub program: String,
//     pub user_id: uuid::Uuid,
//     pub school_id: uuid::Uuid,
//     pub created_at: chrono::NaiveDateTime,
//     pub updated_at: Option<chrono::NaiveDateTime>,
//     pub department: Option<String>,
// }
