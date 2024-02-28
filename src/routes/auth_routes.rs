use std::sync::Arc;

use actix_web::{Responder, web};
use diesel::result::Error;

use crate::configs::common::ApplicationConfig;
use crate::helper::type_alias::DbPool;
use crate::helper::utils::get_connection;
use crate::schemas::auth_schema::LoginRequest;
use crate::services::auth_service::AuthService;

pub struct AuthRoutes;

impl AuthRoutes {
    pub async fn login(
        pool: web::Data<DbPool>,
        auth: web::Json<LoginRequest>,
        app_config: web::Data<ApplicationConfig>,
    ) -> actix_web::Result<impl Responder> {
        log::info!("Logging in: {:?}", auth.email);
        let mut conn = get_connection(&pool).await;

        let token = AuthService::login(&mut conn, auth.into_inner(), &app_config.auth).await;
        match token {
            Ok(token) => Ok(actix_web::HttpResponse::Ok().json(token)),
            Err(e) => {
                log::error!("Failed to login: {}", e);
                Err(actix_web::error::ErrorInternalServerError(e))
            }
        }
    }

    // @TODO: temporary function to test the authentication (decode token)
    pub async fn authenticate(
        pool: web::Data<DbPool>,
        app_config: web::Data<ApplicationConfig>,
        req: actix_web::HttpRequest,
    ) -> actix_web::Result<impl Responder> {
        log::info!("Authenticating user");
        log::info!("Request: {:?}", req);

        let bearer = req.headers().get("Authorization");
        let token = match bearer {
            Some(token) => token.to_str().unwrap().split_whitespace().last().unwrap(),
            None => return Err(actix_web::error::ErrorUnauthorized("No token provided")),
        };
        log::info!("Token: {}", token);

        let payload = AuthService::decode_token(&token, &app_config.auth).await;
        match payload {
            Ok(data) => Ok(actix_web::HttpResponse::Ok().json(data)),
            Err(e) => {
                log::error!("Failed to authenticate: {}", e);
                Err(actix_web::error::ErrorInternalServerError(e))
            }
        }
    }
}
