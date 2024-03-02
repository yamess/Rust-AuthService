use chrono::NaiveTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct ScheduleResponse {
    pub id: Uuid,
    pub student_id: Uuid,
    pub class_id: Uuid,
    pub day_of_week: i16,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ScheduleCreate {
    pub student_id: Uuid,
    pub class_id: Uuid,
    pub day_of_week: i16,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ScheduleUpdate {
    pub student_id: Uuid,
    pub class_id: Uuid,
    pub day_of_week: i16,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
}