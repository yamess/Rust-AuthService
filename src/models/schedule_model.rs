use chrono::{NaiveDateTime, NaiveTime};
use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::schedules;

#[derive(
    Insertable,
    Queryable,
    Identifiable,
    Selectable,
    Deserialize,
    Serialize,
    AsChangeset,
    Debug,
    PartialEq,
)]
#[diesel(table_name = schedules)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ScheduleModel {
    pub id: Uuid,
    pub student_id: Uuid,
    pub class_id: Uuid,
    pub day_of_week: i16,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
}

impl ScheduleModel {
    pub fn new(
        student_id: Uuid,
        class_id: Uuid,
        day_of_week: i16,
        start_time: NaiveTime,
        end_time: NaiveTime,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            student_id,
            class_id,
            day_of_week,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: None,
            start_time,
            end_time,
        }
    }
}
