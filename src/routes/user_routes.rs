use actix_web::{HttpResponse, Responder, web};
use bb8::PooledConnection;
use diesel::Insertable;
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;

use crate::helper::enums::Identifier;
use crate::helper::type_alias::DbPool;
use crate::interfaces::repository_interface::IRepository;
use crate::repositories::user_repository::{UserCreate, UserRepository, UserUpdate};
use crate::tables::users::dsl::users;

pub struct UserRoutes;

impl UserRoutes {
    pub async fn conn(pool: &web::Data<DbPool>) -> PooledConnection<'_,
        AsyncDieselConnectionManager<AsyncPgConnection>> {
        pool.get()
            .await
            .map_err(|e| {
                log::error!("Failed to get pool: {}", e);
                actix_web::error::ErrorInternalServerError(e)
            })
            .unwrap()
    }

    pub async fn create(
        user: web::Json<UserCreate>,
        pool: web::Data<DbPool>,
    ) -> actix_web::Result<impl Responder> {
        log::info!("Creating user: {:?}", user);
        let mut conn = UserRoutes::conn(&pool).await;
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
    ) -> actix_web::Result<impl Responder> {
        let id = Identifier::Id(id.into_inner());
        log::info!("Getting user: {:?}", id);

        let mut conn = UserRoutes::conn(&pool).await;
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
        log::info!("Updating user: {:?}", user);
        let mut conn = UserRoutes::conn(&pool).await;

        let _id = id.into_inner();
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
}