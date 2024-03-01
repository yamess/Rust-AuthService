use std::future::{ready, Future};
use std::pin::Pin;

use actix_web::http::header::HeaderValue;
use actix_web::{http, web, FromRequest, HttpRequest};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::configs::common::ApplicationConfig;

use crate::services::token_service::TokenService;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthExtractorService {
    pub id: Uuid,
    pub tenant_id: Option<Uuid>,
    pub email: String,
    pub admin: bool,
    pub active: bool,
}

impl AuthExtractorService {
    /**
     * Extracts the token from the request header
     *
     * @param req: &HttpRequest
     * @return Result<String, actix_web::Error>
     */
    fn extract_token(req: &HttpRequest) -> Result<String, actix_web::Error> {
        let auth_header: Option<&HeaderValue> = req.headers().get(http::header::AUTHORIZATION);
        if auth_header.is_none() {
            log::error!("No authorization header found");
            return Err(actix_web::error::ErrorUnauthorized("Unauthorized"));
        }
        let token = auth_header
            .unwrap()
            .to_str()
            .unwrap_or("")
            .to_string()
            .replace("Bearer ", "");
        if token.is_empty() {
            log::error!("No token found");
            return Err(actix_web::error::ErrorUnauthorized("Unauthorized"));
        }
        Ok(token)
    }
}

impl FromRequest for AuthExtractorService {
    /**
     * The associated error which can be returned.
     */
    type Error = actix_web::Error;

    /**
     * The future response value.
     */
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    /**
     * Extracts the token from the request header and validates it
     *
     * @param req: &HttpRequest
     * @param _: &mut actix_web::dev::Payload
     * @return Self::Future
     */
    fn from_request(req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let start = std::time::Instant::now();
        let token = match Self::extract_token(req) {
            Ok(token) => token,
            Err(e) => return Box::pin(ready(Err(e))),
        };

        let data = match req.app_data::<web::Data<ApplicationConfig>>() {
            Some(data) => data,
            None => {
                log::error!("No application data found");
                return Box::pin(ready(Err(actix_web::error::ErrorInternalServerError(
                    "Internal Server Error",
                ))));
            }
        };

        let auth_config = data.auth.clone();

        Box::pin(async move {
            let token_claims = TokenService::decode(&token, &auth_config).await;
            match token_claims {
                Ok(claims) => {
                    log::debug!(
                        "Authentication Elapsed time: {:?}ms",
                        start.elapsed().as_millis()
                    );
                    Ok(AuthExtractorService {
                        id: claims.sub,
                        tenant_id: claims.tenant_id,
                        email: claims.email,
                        admin: claims.admin,
                        active: claims.active,
                    })
                }
                Err(e) => {
                    log::debug!(
                        "Authentication Elapsed time: {:?}ms",
                        start.elapsed().as_millis()
                    );
                    log::error!("Failed to authenticate token: {}", e);
                    return Err(actix_web::error::ErrorUnauthorized("Unauthorized"));
                }
            }
        })
    }
}
