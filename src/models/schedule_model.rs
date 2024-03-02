use chrono::{NaiveDateTime, NaiveTime};
use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}