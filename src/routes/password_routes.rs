use actix_web::{web, HttpResponse, Responder};

use crate::helper::enums::Identifier;
use crate::helper::type_alias::DbPool;
use crate::helper::utils::get_connection;
use crate::schemas::user_schemas::PasswordUpdate;
use crate::services::auth_extractor::AuthExtractorService;
use crate::services::password_service::PasswordService;

pub struct PasswordRoutes;

impl PasswordRoutes {
    pub async fn update(
        pool: web::Data<DbPool>,
        id: web::Path<uuid::Uuid>,
        password: web::Json<PasswordUpdate>,
        _: AuthExtractorService,
    ) -> actix_web::Result<impl Responder> {
        let mut conn = get_connection(&pool).await;
        let _id = id.into_inner();
        log::info!("Updating password for user: {:?}", &_id);

        let updated_password = PasswordService::update_password(
            &mut conn,
            &Identifier::Id(_id),
            password.into_inner(),
        )
        .await;

        match updated_password {
            Ok(_) => {
                log::info!("Password updated for user: {:?}", &_id);
                Ok(HttpResponse::Ok().finish())
            }
            Err(e) => {
                log::error!("Failed to update password: {}", e);
                Err(actix_web::error::ErrorInternalServerError(e))
            }
        }
    }
}
