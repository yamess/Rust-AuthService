use actix_web::web;
use uuid::Uuid;

use crate::helper::enums::Identifier;
use crate::helper::type_alias::DbPool;
use crate::helper::utils::get_connection;
use crate::interfaces::repository_interface::IRepository;
use crate::repositories::student_repository::StudentRepository;
use crate::schemas::student_schemas::{StudentCreate, StudentUpdate};

pub struct StudentRoutes;

impl StudentRoutes {
    pub async fn create(
        pool: web::Data<DbPool>,
        student: web::Json<StudentCreate>,
    ) -> actix_web::Result<impl actix_web::Responder> {
        log::info!("Creating student account for user: {:?}", student.user_id);
        let mut conn = get_connection(&pool).await;

        let student = StudentRepository::create(&mut conn, student.into_inner()).await;
        match student {
            Ok(student) => Ok(actix_web::HttpResponse::Ok().json(student)),
            Err(e) => {
                log::error!("Failed to create student: {}", e);
                Err(actix_web::error::ErrorInternalServerError(e))
            }
        }
    }
    pub async fn get(
        pool: web::Data<DbPool>,
        id: web::Path<Uuid>,
    ) -> actix_web::Result<impl actix_web::Responder> {
        let mut conn = get_connection(&pool).await;
        let _id = Identifier::Id(id.into_inner());
        let student = StudentRepository::get(&mut conn, &_id).await;
        match student {
            Ok(student) => Ok(actix_web::HttpResponse::Ok().json(student)),
            Err(e) => {
                log::error!("Failed to get student: {}", e);
                Err(actix_web::error::ErrorInternalServerError(e))
            }
        }
    }

    pub async fn update(
        pool: web::Data<DbPool>,
        id: web::Path<Uuid>,
        student: web::Json<StudentUpdate>,
    ) -> actix_web::Result<impl actix_web::Responder> {
        let mut conn = get_connection(&pool).await;
        let _id = Identifier::Id(id.into_inner());
        log::info!("Updating student: {:?}", &_id);
        let student = student.into_inner();
        let student = StudentRepository::update(&mut conn, &_id, student).await;
        match student {
            Ok(student) => Ok(actix_web::HttpResponse::Ok().json(student)),
            Err(e) => {
                log::error!("Failed to update student: {}", e);
                Err(actix_web::error::ErrorInternalServerError(e))
            }
        }
    }

    pub async fn delete(
        pool: web::Data<DbPool>,
        id: web::Path<Uuid>,
    ) -> actix_web::Result<impl actix_web::Responder> {
        let mut conn = get_connection(&pool).await;
        let _id = Identifier::Id(id.into_inner());
        log::info!("Deleting student: {:?}", &_id);
        let deleted_student = StudentRepository::delete(&mut conn, &_id).await;
        match deleted_student {
            Ok(num) => Ok(actix_web::HttpResponse::Ok().json(num)),
            Err(e) => {
                log::error!("Failed to delete student: {}", e);
                Err(actix_web::error::ErrorInternalServerError(e))
            }
        }
    }
}
