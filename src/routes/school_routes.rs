use actix_web::{web, Responder};

use crate::helper::enums::Identifier;
use crate::helper::type_alias::DbPool;
use crate::helper::utils::get_connection;
use crate::interfaces::repository_interface::IRepository;
use crate::repositories::school_repository::SchoolRepository;
use crate::schemas::school_schemas::{SchoolCreate, SchoolUpdate};

pub struct SchoolRoutes;

impl SchoolRoutes {
    pub async fn create(
        pool: web::Data<DbPool>,
        school: web::Json<SchoolCreate>,
    ) -> actix_web::Result<impl actix_web::Responder> {
        log::info!("Creating school: {:?}", school.name);
        let mut conn = get_connection(&pool).await;

        let _school = SchoolRepository::create(&mut conn, school.into_inner()).await;
        match _school {
            Ok(_school) => Ok(actix_web::HttpResponse::Ok().json(_school)),
            Err(e) => {
                log::error!("Failed to create school: {}", e);
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
        let school = SchoolRepository::get(&mut conn, &_id).await;

        match school {
            Ok(school) => Ok(actix_web::HttpResponse::Ok().json(school)),
            Err(e) => {
                log::error!("Failed to get school: {}", e);
                Err(actix_web::error::ErrorInternalServerError(e))
            }
        }
    }

    pub async fn update(
        pool: web::Data<DbPool>,
        id: web::Path<uuid::Uuid>,
        school: web::Json<SchoolUpdate>,
    ) -> actix_web::Result<impl Responder> {
        let mut conn = get_connection(&pool).await;
        let _id = Identifier::Id(id.into_inner());
        log::info!("Updating school: {:?}", &_id);

        let school = school.into_inner();
        let updated_school = SchoolRepository::update(&mut conn, &_id, school).await;

        match updated_school {
            Ok(updated_school) => Ok(actix_web::HttpResponse::Ok().json(updated_school)),
            Err(e) => {
                log::error!("Failed to update school: {}", e);
                Err(actix_web::error::ErrorInternalServerError(e))
            }
        }
    }

    pub async fn delete(
        pool: web::Data<DbPool>,
        id: web::Path<uuid::Uuid>,
    ) -> actix_web::Result<impl Responder> {
        let mut conn = get_connection(&pool).await;
        let _id = Identifier::Id(id.into_inner());
        log::info!("Deleting school: {:?}", &_id);

        let deleted_school = SchoolRepository::delete(&mut conn, &_id).await;

        match deleted_school {
            Ok(deleted_school) => Ok(actix_web::HttpResponse::Ok().json(deleted_school)),
            Err(e) => {
                log::error!("Failed to delete school: {}", e);
                Err(actix_web::error::ErrorInternalServerError(e))
            }
        }
    }
}
