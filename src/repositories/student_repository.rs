use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::result::Error;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::helper::enums::Identifier;
use crate::helper::utils::type_of;
use crate::interfaces::repository_interface::IRepository;
use crate::models::student_model::StudentModel;
use crate::schema::students;
use crate::schemas::student_schemas::{StudentCreate, StudentResponse, StudentUpdate};

pub struct StudentRepository;

impl IRepository<'_, StudentCreate, StudentUpdate, StudentResponse> for StudentRepository {
    type Model = StudentModel;

    async fn create(
        conn: &mut AsyncPgConnection,
        data: StudentCreate,
    ) -> Result<StudentResponse, Error> {
        let new_student = Self::Model::new(
            data.first_name,
            data.last_name,
            data.program,
            data.department,
            data.user_id,
            data.school_id,
            chrono::Utc::now().naive_utc(),
            None,
        );
        let created_student = diesel::insert_into(crate::schema::students::table)
            .values(&new_student)
            .get_result::<Self::Model>(conn)
            .await;
        match created_student {
            Err(e) => {
                log::error!("Failed to create student: {}", e);
                Err(e)
            }
            Ok(created_student) => Ok(StudentResponse {
                id: created_student.id,
                first_name: created_student.first_name,
                last_name: created_student.last_name,
                program: created_student.program,
                department: created_student.department,
                user_id: created_student.user_id,
                school_id: created_student.school_id,
                created_at: created_student.created_at,
                updated_at: created_student.updated_at,
            }),
        }
    }

    async fn get(
        conn: &mut AsyncPgConnection,
        id: &Identifier,
    ) -> Result<Option<StudentResponse>, Error> {
        let student = match id {
            Identifier::Int(id) => crate::schema::students::table
                .find(id)
                .get_result::<Self::Model>(conn)
                .await
                .map(Some),
            _ => Err(Error::NotFound),
        };

        match student {
            Ok(None) => {
                log::error!("Student not found");
                Ok(None)
            }
            Err(e) => {
                log::error!("Failed to get student: {}", e);
                Err(e)
            }
            Ok(Some(student)) => Ok(Some(StudentResponse {
                id: student.id,
                first_name: student.first_name,
                last_name: student.last_name,
                program: student.program,
                department: student.department,
                user_id: student.user_id,
                school_id: student.school_id,
                created_at: student.created_at,
                updated_at: student.updated_at,
            })),
        }
    }

    async fn update(
        conn: &mut AsyncPgConnection,
        id: &Identifier,
        new_data: StudentUpdate,
    ) -> Result<StudentResponse, Error> {
        let old_data = match id {
            Identifier::Int(id) => {
                students::table
                    .find(id)
                    .get_result::<Self::Model>(conn)
                    .await?
            }
            _ => {
                log::error!(
                    "Wrong student identifier. Expecting int type. Got {:?}",
                    type_of(id)
                );
                return Err(Error::NotFound);
            }
        };

        let updated_student = diesel::update(&old_data)
            .set((
                students::first_name.eq(new_data.first_name),
                students::last_name.eq(new_data.last_name),
                students::program.eq(new_data.program),
                students::department.eq(new_data.department),
                students::school_id.eq(new_data.school_id),
            ))
            .get_result::<Self::Model>(conn)
            .await;


        match updated_student {
            Err(e) => {
                log::error!("Failed to update student: {}", e);
                Err(e)
            }
            Ok(updated_student) => Ok(StudentResponse {
                id: updated_student.id,
                first_name: updated_student.first_name,
                last_name: updated_student.last_name,
                program: updated_student.program,
                department: updated_student.department,
                user_id: updated_student.user_id,
                school_id: updated_student.school_id,
                created_at: updated_student.created_at,
                updated_at: updated_student.updated_at,
            }),
        }
    }

    async fn delete(conn: &mut AsyncPgConnection, id: &Identifier) -> Result<usize, Error> {
        let number_deleted = match id {
            Identifier::Int(id) => {
                diesel::delete(crate::schema::students::table.find(id))
                    .execute(conn)
                    .await
            }
            _ => {
                log::error!(
                    "Wrong student identifier. Expecting int type. Got {:?}",
                    type_of(id)
                );
                Err(Error::NotFound)
            }
        };
        match number_deleted {
            Ok(number_deleted) => Ok(number_deleted),
            Err(e) => {
                log::error!("Failed to delete student: {}", e);
                Err(e)
            }
        }
    }
}
