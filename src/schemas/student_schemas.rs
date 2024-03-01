use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct StudentResponse {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub program: String,
    pub department: Option<String>,
    pub user_id: uuid::Uuid,
    pub school_id: uuid::Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StudentCreate {
    pub first_name: String,
    pub last_name: String,
    pub program: String,
    pub department: Option<String>,
    pub user_id: uuid::Uuid,
    pub school_id: uuid::Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StudentUpdate {
    pub first_name: String,
    pub last_name: String,
    pub program: String,
    pub department: Option<String>,
    pub school_id: uuid::Uuid,
}
