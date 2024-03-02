use actix_web::{Responder, web};
use uuid::Uuid;

use crate::helper::enums::Identifier;
use crate::helper::type_alias::DbPool;
use crate::helper::utils::get_connection;
use crate::interfaces::repository_interface::IRepository;
use crate::repositories::class_repository::ClassRepository;
use crate::schemas::class_schema::{ClassCreate, ClassUpdate};

pub struct ClassRoutes;

impl ClassRoutes {
    pub async fn create(
        pool: web::Data<DbPool>,
        class: web::Json<ClassCreate>,
    ) -> actix_web::Result<impl Responder> {
        log::info!("Creating class: {:?}", class.name);
        let mut conn = get_connection(&pool).await;
        let _class = ClassRepository::create(&mut conn, class.into_inner()).await;
        match _class {
            Ok(_class) => Ok(actix_web::HttpResponse::Ok().json(_class)),
            Err(e) => {
                log::error!("Failed to create class: {}", e);
                Err(actix_web::error::ErrorInternalServerError(e))
            }
        }
    }

    pub async fn get(
        pool: web::Data<DbPool>,
        id: web::Path<uuid::Uuid>,
    ) -> actix_web::Result<impl Responder> {
        let mut conn = get_connection(&pool).await;
        let _id = Identifier::Id(id.into_inner());
        let class = ClassRepository::get(&mut conn, &_id).await;
        match class {
            Ok(class) => Ok(actix_web::HttpResponse::Ok().json(class)),
            Err(e) => {
                log::error!("Failed to get class: {}", e);
                Err(actix_web::error::ErrorInternalServerError(e))
            }
        }
    }

    pub async fn update(
        pool: web::Data<DbPool>,
        id: web::Path<uuid::Uuid>,
        class: web::Json<ClassUpdate>,
    ) -> actix_web::Result<impl Responder> {
        let mut conn = get_connection(&pool).await;
        let _id = Identifier::Id(id.into_inner());
        log::info!("Updating class: {:?}", &_id);

        let class = class.into_inner();
        let updated_class = ClassRepository::update(&mut conn, &_id, class).await;
        match updated_class {
            Ok(updated_class) => Ok(actix_web::HttpResponse::Ok().json(updated_class)),
            Err(e) => {
                log::error!("Failed to update class: {}", e);
                Err(actix_web::error::ErrorInternalServerError(e))
            }
        }
    }

    pub async fn delete(
        pool: web::Data<DbPool>,
        id: web::Path<Uuid>,
    ) -> actix_web::Result<impl Responder> {
        let mut conn = get_connection(&pool).await;
        let _id = Identifier::Id(id.into_inner());
        log::info!("Deleting class: {:?}", &_id);

        let deleted_class = ClassRepository::delete(&mut conn, &_id).await;
        match deleted_class {
            Ok(deleted_class) => Ok(actix_web::HttpResponse::Ok().json(deleted_class)),
            Err(e) => {
                log::error!("Failed to delete class: {}", e);
                Err(actix_web::error::ErrorInternalServerError(e))
            }
        }
    }
}