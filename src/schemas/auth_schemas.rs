use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub exp: i64,
    pub iat: i64,
    pub sub: Uuid,
    pub email: String,
    pub tenant_id: Option<Uuid>,
    pub admin: bool,
    pub active: bool,
}
