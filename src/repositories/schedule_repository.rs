use diesel::result::Error;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::helper::enums::Identifier;
use crate::helper::utils::type_of;
use crate::interfaces::repository_interface::IRepository;
use crate::models::schedule_model::ScheduleModel;
use crate::schema::schedules;
use crate::schemas::schedule_schemas::{ScheduleCreate, ScheduleResponse, ScheduleUpdate};

pub struct ScheduleRepository;

impl IRepository<'_, ScheduleCreate, ScheduleUpdate, ScheduleResponse> for ScheduleRepository {
    type Model = ScheduleModel;

    async fn create(
        conn: &mut AsyncPgConnection,
        data: ScheduleCreate,
    ) -> Result<ScheduleResponse, Error> {
        let new_schedule = Self::Model::new(
            data.student_id,
            data.class_id,
            data.day_of_week,
            data.start_time,
            data.end_time,
        );
        let created_schedule = diesel::insert_into(schedules::table)
            .values(&new_schedule)
            .get_result::<Self::Model>(conn)
            .await;
        match created_schedule {
            Err(e) => {
                log::error!("Failed to create schedule: {}", e);
                Err(e)
            }
            Ok(created_schedule) => Ok(ScheduleResponse {
                id: created_schedule.id,
                student_id: created_schedule.student_id,
                class_id: created_schedule.class_id,
                day_of_week: created_schedule.day_of_week,
                start_time: created_schedule.start_time,
                end_time: created_schedule.end_time,
                created_at: created_schedule.created_at,
                updated_at: created_schedule.updated_at,
            }),
        }
    }

    async fn get(
        conn: &mut AsyncPgConnection,
        id: &Identifier,
    ) -> Result<Option<ScheduleResponse>, Error> {
        let schedule = match id {
            Identifier::Id(id) => schedules::table
                .find(id)
                .get_result::<Self::Model>(conn)
                .await
                .map(Some),
            _ => {
                log::error!(
                    "Wrong schedule identifier. Expecting int type. Got {:?}",
                    type_of(id)
                );
                Err(Error::NotFound)
            }
        };

        match schedule {
            Err(e) => {
                log::error!("Failed to get schedule: {}", e);
                Err(e)
            }
            Ok(None) => {
                log::error!("Schedule id {:?} not found", id);
                Ok(None)
            }
            Ok(Some(schedule)) => Ok(Some(ScheduleResponse {
                id: schedule.id,
                student_id: schedule.student_id,
                class_id: schedule.class_id,
                day_of_week: schedule.day_of_week,
                start_time: schedule.start_time,
                end_time: schedule.end_time,
                created_at: schedule.created_at,
                updated_at: schedule.updated_at,
            })),
        }
    }

    async fn update(
        conn: &mut AsyncPgConnection,
        id: &Identifier,
        new_data: ScheduleUpdate,
    ) -> Result<ScheduleResponse, Error> {
        let old_data = match id {
            Identifier::Id(id) => {
                schedules::table
                    .find(id)
                    .get_result::<Self::Model>(conn)
                    .await?
            }
            _ => {
                log::error!(
                    "Wrong schedule identifier. Expecting int type. Got {:?}",
                    type_of(id)
                );
                Err(Error::NotFound)?
            }
        };

        let updated_schedule = diesel::update(&old_data)
            .set((
                schedules::student_id.eq(new_data.student_id),
                schedules::class_id.eq(new_data.class_id),
                schedules::day_of_week.eq(new_data.day_of_week),
                schedules::start_time.eq(new_data.start_time),
                schedules::end_time.eq(new_data.end_time),
            ))
            .get_result::<Self::Model>(conn)
            .await;

        match updated_schedule {
            Err(e) => {
                log::error!("Failed to update schedule: {}", e);
                Err(e)
            }
            Ok(updated_schedule) => Ok(ScheduleResponse {
                id: updated_schedule.id,
                student_id: updated_schedule.student_id,
                class_id: updated_schedule.class_id,
                day_of_week: updated_schedule.day_of_week,
                start_time: updated_schedule.start_time,
                end_time: updated_schedule.end_time,
                created_at: updated_schedule.created_at,
                updated_at: updated_schedule.updated_at,
            }),
        }
    }

    async fn delete(conn: &mut AsyncPgConnection, id: &Identifier) -> Result<usize, Error> {
        let deleted_schedule = match id {
            Identifier::Id(id) => {
                diesel::delete(schedules::table.find(id))
                    .execute(conn)
                    .await
            }
            _ => {
                log::error!(
                    "Wrong schedule identifier. Expecting int type. Got {:?}",
                    type_of(id)
                );
                Err(Error::NotFound)?
            }
        };
        match deleted_schedule {
            Err(e) => {
                log::error!("Failed to delete schedule: {}", e);
                Err(e)
            }
            Ok(deleted_schedule) => Ok(deleted_schedule),
        }
    }
}
