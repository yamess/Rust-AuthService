use actix_web::web;

use crate::helper::enums::Identifier;
use crate::helper::type_alias::DbPool;
use crate::helper::utils::get_connection;
use crate::interfaces::repository_interface::IRepository;
use crate::repositories::schedule_repository::ScheduleRepository;

pub struct ScheduleRoutes;

impl ScheduleRoutes {
    pub async fn create(
        pool: web::Data<DbPool>,
        schedule: web::Json<crate::schemas::schedule_schemas::ScheduleCreate>,
    ) -> actix_web::Result<impl actix_web::Responder> {
        log::info!("Creating new schedule for student: {:?}", schedule.student_id);
        let mut conn = get_connection(&pool).await;
        let schedule = ScheduleRepository::create(&mut conn, schedule.into_inner()).await;
        match schedule {
            Ok(schedule) => Ok(actix_web::HttpResponse::Ok().json(schedule)),
            Err(e) => {
                log::error!("Failed to create schedule: {}", e);
                Err(actix_web::error::ErrorInternalServerError(e))
            }
        }
    }

    pub async fn get(
        pool: web::Data<DbPool>,
        id: web::Path<uuid::Uuid>,
    ) -> actix_web::Result<impl actix_web::Responder> {
        let mut conn = get_connection(&pool).await;
        let id = Identifier::Id(id.into_inner());
        let schedule = ScheduleRepository::get(&mut conn, &id).await;
        match schedule {
            Ok(schedule) => Ok(actix_web::HttpResponse::Ok().json(schedule)),
            Err(e) => {
                log::error!("Failed to get schedule: {}", e);
                Err(actix_web::error::ErrorInternalServerError(e))
            }
        }
    }

    pub async fn update(
        pool: web::Data<DbPool>,
        id: web::Path<uuid::Uuid>,
        schedule: web::Json<crate::schemas::schedule_schemas::ScheduleUpdate>,
    ) -> actix_web::Result<impl actix_web::Responder> {
        let mut conn = get_connection(&pool).await;
        let id = Identifier::Id(id.into_inner());
        log::info!("Updating schedule: {:?}", &id);
        let schedule = schedule.into_inner();
        let schedule = ScheduleRepository::update(&mut conn, &id, schedule).await;
        match schedule {
            Ok(schedule) => Ok(actix_web::HttpResponse::Ok().json(schedule)),
            Err(e) => {
                log::error!("Failed to update schedule: {}", e);
                Err(actix_web::error::ErrorInternalServerError(e))
            }
        }
    }

    pub async fn delete(
        pool: web::Data<DbPool>,
        id: web::Path<uuid::Uuid>,
    ) -> actix_web::Result<impl actix_web::Responder> {
        let mut conn = get_connection(&pool).await;
        let id = Identifier::Id(id.into_inner());
        log::info!("Deleting schedule: {:?}", &id);
        let deleted_schedule = ScheduleRepository::delete(&mut conn, &id).await;
        match deleted_schedule {
            Ok(schedule) => Ok(actix_web::HttpResponse::Ok().json(schedule)),
            Err(e) => {
                log::error!("Failed to delete schedule: {}", e);
                Err(actix_web::error::ErrorInternalServerError(e))
            }
        }
    }
}