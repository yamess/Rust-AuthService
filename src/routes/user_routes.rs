use actix_web::{web, HttpResponse, Responder};

use crate::helper::type_alias::DbPool;
use crate::interfaces::repository_interface::IRepository;
use crate::repositories::user_repository::{UserCreate, UserRepository};

pub async fn create_user(
    user: web::Json<UserCreate>,
    pool: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    log::info!("Creating user: {:?}", user);
    let mut conn = pool
        .get()
        .await
        .map_err(|e| {
            log::error!("Failed to get pool: {}", e);
            actix_web::error::ErrorInternalServerError(e)
        })
        .unwrap();
    let _user = UserRepository::create(&mut conn, user.into_inner()).await;
    match _user {
        Ok(_user) => Ok(HttpResponse::Ok().json(_user)),
        Err(e) => {
            log::error!("Failed to create user: {}", e);
            Err(actix_web::error::ErrorInternalServerError(e))
        }
    }
}
