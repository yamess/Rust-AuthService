use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TokenClaims {
    pub aud: Option<String>,
    pub exp: i64,
    pub iat: i64,
    pub iss: String,
    pub nbf: i64,
    pub sub: String,
    pub email: String,
}
