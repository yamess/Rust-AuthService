use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClassResponse {
    pub id: Uuid,
    pub name: String,
    pub student_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClassCreate {
    pub name: String,
    pub student_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClassUpdate {
    pub name: String,
    pub student_id: Uuid,
}
