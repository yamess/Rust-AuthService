use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SchoolResponse {
    pub id: uuid::Uuid,
    pub name: String,
    pub website: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SchoolCreate {
    pub name: String,
    pub website: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SchoolUpdate {
    pub name: String,
    pub website: String,
}