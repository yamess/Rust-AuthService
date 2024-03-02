use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::result::Error;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::helper::enums::Identifier;
use crate::helper::utils::type_of;
use crate::interfaces::repository_interface::IRepository;
use crate::models::class_model::ClassModel;
use crate::schema::classes;
use crate::schemas::class_schema::{ClassCreate, ClassResponse, ClassUpdate};

pub struct ClassRepository;

impl IRepository<'_, ClassCreate, ClassUpdate, ClassResponse> for ClassRepository {
    type Model = ClassModel;

    async fn create(conn: &mut AsyncPgConnection, data: ClassCreate) -> Result<ClassResponse, Error> {
        let new_class = Self::Model::new(data.name, data.student_id);
        let created_class = diesel::insert_into(classes::table)
            .values(&new_class)
            .get_result::<Self::Model>(conn)
            .await;
        match created_class {
            Err(e) => {
                log::error!("Failed to create class: {}", e);
                Err(e)
            }
            Ok(created_class) => Ok(ClassResponse {
                id: created_class.id,
                name: created_class.name,
                student_id: created_class.student_id,
                created_at: created_class.created_at,
                updated_at: created_class.updated_at,
            }),
        }
    }

    async fn get(conn: &mut AsyncPgConnection, id: &Identifier) -> Result<Option<ClassResponse>, Error> {
        let class = match id {
            Identifier::Id(id) => classes::table
                .find(id)
                .get_result::<Self::Model>(conn)
                .await
                .map(Some),
            _ => {
                log::error!(
                        "Wrong class identifier. Expecting int type. Got {:?}",
                        type_of(id)
                    );
                Err(Error::NotFound)
            }
        };

        match class {
            Err(e) => {
                log::error!("Failed to get class: {}", e);
                Err(e)
            }
            Ok(None) => {
                log::error!("Class not found");
                Ok(None)
            }
            Ok(Some(class)) => Ok(Some(ClassResponse {
                id: class.id,
                name: class.name,
                student_id: class.student_id,
                created_at: class.created_at,
                updated_at: class.updated_at,
            })),
        }
    }

    async fn update(conn: &mut AsyncPgConnection, id: &Identifier, new_data: ClassUpdate) -> Result<ClassResponse, Error> {
        let old_data = match id {
            Identifier::Id(id) => classes::table
                .find(id)
                .get_result::<Self::Model>(conn)
                .await?,
            _ => {
                log::error!(
                    "Wrong class identifier. Expecting int type. Got {:?}",
                    type_of(id)
                );
                Err(Error::NotFound)?
            }
        };

        let updated_class = diesel::update(&old_data)
            .set((
                classes::name.eq(new_data.name),
                classes::student_id.eq(new_data.student_id),
            ))
            .get_result::<Self::Model>(conn)
            .await;

        match updated_class {
            Err(e) => {
                log::error!("Failed to update class: {}", e);
                Err(e)
            }
            Ok(updated_class) => Ok(ClassResponse {
                id: updated_class.id,
                name: updated_class.name,
                student_id: updated_class.student_id,
                created_at: updated_class.created_at,
                updated_at: updated_class.updated_at,
            }),
        }
    }

    async fn delete(conn: &mut AsyncPgConnection, id: &Identifier) -> Result<usize, Error> {
        let number_deleted = match id {
            Identifier::Id(id) => diesel::delete(classes::table.find(id)).execute(conn).await,
            _ => {
                log::error!("Wrong class identifier. Expecting uuid type. Got {:?}",type_of(id));
                Err(Error::NotFound)?
            }
        };
        match number_deleted {
            Err(e) => {
                log::error!("Failed to delete class: {}", e);
                Err(e)
            }
            Ok(num) => Ok(num),
        }
    }
}