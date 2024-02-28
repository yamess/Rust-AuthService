use actix_web::{HttpResponse, Responder, web};
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use utoipa;
use uuid::{uuid, Uuid};

use crate::helper::enums::Identifier;
use crate::helper::type_alias::DbPool;
use crate::helper::utils::get_connection;
use crate::interfaces::repository_interface::IRepository;
use crate::repositories::user_repository::UserRepository;
use crate::schemas::user_schemas::{UserCreate, UserUpdate};
use crate::services::auth_extractor_service::AuthExtractorService;
use crate::tables::users::dsl::users;

pub struct UserRoutes;

impl UserRoutes {
    pub async fn create(
        user: web::Json<UserCreate>,
        pool: web::Data<DbPool>,
    ) -> actix_web::Result<impl Responder> {
        log::info!("Creating user: {:?}", user.email);
        let mut conn = get_connection(&pool).await;
        let _user = UserRepository::create(&mut conn, user.into_inner()).await;
        match _user {
            Ok(_user) => Ok(HttpResponse::Ok().json(_user)),
            Err(e) => {
                log::error!("Failed to create user: {}", e);
                Err(actix_web::error::ErrorInternalServerError(e))
            }
        }
    }

    pub async fn get(
        pool: web::Data<DbPool>,
        id: web::Path<uuid::Uuid>,
        auth: AuthExtractorService,
    ) -> actix_web::Result<impl Responder> {
        let id = Identifier::Id(id.into_inner());
        log::info!("Getting user: {:?}", id);

        let mut conn = get_connection(&pool).await;
        let _user = UserRepository::get(&mut conn, &id).await;
        match _user {
            Ok(_user) => Ok(HttpResponse::Ok().json(_user)),
            Err(e) => {
                log::error!("Failed to get user: {}", e);
                Err(actix_web::error::ErrorInternalServerError(e))
            }
        }
    }

    pub async fn update(
        pool: web::Data<DbPool>,
        id: web::Path<uuid::Uuid>,
        user: web::Json<UserUpdate>,
    ) -> actix_web::Result<impl Responder> {
        let mut conn = get_connection(&pool).await;
        let _id = id.into_inner();
        log::info!("Updating user: {:?}", &_id);

        let user = user.into_inner();
        let updated_user = UserRepository::update(&mut conn, &Identifier::Id(_id), user).await;

        match updated_user {
            Ok(_user) => Ok(HttpResponse::Ok().json(_user)),
            Err(e) => {
                log::error!("Failed to update user: {}", e);
                Err(actix_web::error::ErrorInternalServerError(e))
            }
        }
    }

    pub async fn delete(
        pool: web::Data<DbPool>,
        id: web::Path<uuid::Uuid>,
    ) -> actix_web::Result<impl Responder> {
        let mut conn = get_connection(&pool).await;
        let _id = id.into_inner();
        log::info!("Deleting user: {:?}", &_id);

        let deletion_count = UserRepository::delete(&mut conn, &Identifier::Id(_id)).await;
        log::info!("deletion_count: {:?}", deletion_count);
        match deletion_count {
            Ok(_count) => {
                if _count == 1 {
                    Ok(HttpResponse::Ok())
                } else {
                    Ok(HttpResponse::NotFound())
                }
            }
            Err(e) => {
                log::error!("Failed to delete user: {}", e);
                Err(actix_web::error::ErrorInternalServerError(e))
            }
        }
    }
}
