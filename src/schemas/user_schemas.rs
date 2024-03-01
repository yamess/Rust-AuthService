use std::fmt::Display;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub is_active: bool,
    pub is_admin: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserCreate {
    pub email: String,
    pub password: String,
    pub is_active: bool,
    pub is_admin: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordUpdate {
    pub old_password: String,
    pub new_password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserUpdate {
    pub is_active: bool,
    pub is_admin: bool,
}
