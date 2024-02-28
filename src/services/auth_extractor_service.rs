use std::future::{Future, Ready, ready};
use std::pin::Pin;

use actix_web::{FromRequest, http, HttpRequest, web};
use actix_web::http::header::HeaderValue;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::configs::common::ApplicationConfig;
use crate::schemas::auth_schema::TokenClaims;
use crate::services::auth_service::AuthService;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthExtractorService {
    pub id: Uuid,
    pub email: String,
}

impl FromRequest for AuthExtractorService {
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output=Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let auth_header: Option<&HeaderValue> = req.headers().get(http::header::AUTHORIZATION);
        log::debug!("Auth header: {:?}", auth_header);
        let token = auth_header.unwrap().to_str().unwrap_or("").to_string().replace("Bearer ", "");
        log::debug!("Auth token: {:?}", token);
        let data = req.app_data::<web::Data<ApplicationConfig>>()
            .expect("Failed to get ApplicationConfig");
        let auth_config = data.auth.clone();


        Box::pin(async move {
            // let auth_config = req.app_data::<web::Data<ApplicationConfig>>().unwrap().auth.clone();

            if token.is_empty() {
                return Err(actix_web::error::ErrorUnauthorized("Unauthorized"));
            }
            let token_claims = AuthService::decode_token(&token, &auth_config)
                .await;
            match token_claims {
                Ok(claims) => {
                    Ok(AuthExtractorService {
                        id: Uuid::parse_str(&claims.sub).unwrap(),
                        email: claims.email,
                    })
                }
                Err(e) => {
                    log::error!("Failed to authenticate token: {}", e);
                    return Err(actix_web::error::ErrorUnauthorized("Unauthorized"));
                }
            }
        })
    }
}