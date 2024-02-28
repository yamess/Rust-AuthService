use serde::{Deserialize, Serialize};
use utoipa::IntoParams;

#[derive(Debug, Serialize, Deserialize, IntoParams)]
pub struct SchoolResponse {
    pub id: uuid::Uuid,
    pub name: String,
    pub website: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, IntoParams)]
pub struct SchoolCreate {
    pub name: String,
    pub website: String,
}

#[derive(Debug, Serialize, Deserialize, IntoParams)]
pub struct SchoolUpdate {
    pub name: String,
    pub website: String,
}
