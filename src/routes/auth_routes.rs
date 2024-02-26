use std::sync::Arc;

use actix_web::{Responder, web};
use diesel::result::Error;

use crate::configs::common::{ApplicationConfig, AuthConfig};
use crate::helper::type_alias::DbPool;
use crate::helper::utils::get_connection;
use crate::schemas::auth_schema::LoginRequest;
use crate::services::auth_service::AuthService;

pub struct AuthRoutes;

impl AuthRoutes {
    pub async fn login(
        pool: web::Data<DbPool>,
        auth: web::Json<LoginRequest>,
        app_config: web::Data<Arc<ApplicationConfig>>,
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
}
