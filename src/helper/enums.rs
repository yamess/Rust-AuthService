use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub enum Identifier {
    Id(Uuid),
    Email(String),
    Int(i32),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UserStatus {
    Inactive = 0,
    Active = 1,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UserRole {
    User = 0,
    Admin = 1,
}
