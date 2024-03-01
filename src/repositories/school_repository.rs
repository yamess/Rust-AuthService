use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::result::Error;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::helper::enums::Identifier;
use crate::helper::utils::type_of;
use crate::interfaces::repository_interface::IRepository;
use crate::models::school_model::SchoolModel;
use crate::schema::schools;
use crate::schemas::school_schemas::{SchoolCreate, SchoolResponse, SchoolUpdate};

pub struct SchoolRepository;

impl IRepository<'_, SchoolCreate, SchoolUpdate, SchoolResponse> for SchoolRepository {
    type Model = SchoolModel;

    async fn create(
        conn: &mut AsyncPgConnection,
        data: SchoolCreate,
    ) -> Result<SchoolResponse, Error> {
        let new_school = Self::Model::new(data.name, data.website);
        let created_school = diesel::insert_into(schools::table)
            .values(&new_school)
            .get_result::<Self::Model>(conn)
            .await;
        match created_school {
            Err(e) => {
                log::error!("Failed to create school: {}", e);
                Err(e)
            }
            Ok(created_school) => Ok(SchoolResponse {
                id: created_school.id,
                name: created_school.name,
                website: created_school.website,
                created_at: created_school.created_at,
                updated_at: created_school.updated_at,
            }),
        }
    }

    async fn get(
        conn: &mut AsyncPgConnection,
        id: &Identifier,
    ) -> Result<Option<SchoolResponse>, Error> {
        let school = match id {
            Identifier::Id(id) => schools::table
                .find(id)
                .get_result::<Self::Model>(conn)
                .await
                .map(Some),
            _ => {
                log::error!(
                    "Wrong student identifier. Expecting int type. Got {:?}",
                    type_of(id)
                );
                Err(Error::NotFound)
            }
        };

        match school {
            Err(e) => {
                log::error!("Failed to get school: {}", e);
                Err(e)
            }
            Ok(None) => {
                log::error!("School id {:?} not found", id);
                Ok(None)
            }
            Ok(Some(school)) => Ok(Some(SchoolResponse {
                id: school.id,
                name: school.name,
                website: school.website,
                created_at: school.created_at,
                updated_at: school.updated_at,
            }))
        }
    }

    async fn update(
        conn: &mut AsyncPgConnection,
        id: &Identifier,
        new_data: SchoolUpdate,
    ) -> Result<SchoolResponse, Error> {
        let old_data = match id {
            Identifier::Id(id) => {
                schools::table
                    .find(id)
                    .get_result::<Self::Model>(conn)
                    .await?
            }
            _ => {
                log::error!("Wrong school identifier. Expecting uuid type. Got {:?}", id);
                Err(Error::NotFound)?
            }
        };

        let updated_school = diesel::update(&old_data)
            .set((
                schools::name.eq(new_data.name),
                schools::website.eq(new_data.website),
                schools::updated_at.eq(chrono::Utc::now().naive_utc()),
            ))
            .get_result::<Self::Model>(conn)
            .await;

        match updated_school {
            Err(e) => {
                log::error!("Failed to update school: {}", e);
                Err(e)
            }
            Ok(sch) => Ok(SchoolResponse {
                id: sch.id,
                name: sch.name,
                website: sch.website,
                created_at: sch.created_at,
                updated_at: sch.updated_at,
            }),
        }
    }

    async fn delete(conn: &mut AsyncPgConnection, id: &Identifier) -> Result<usize, Error> {
        let number_deleted = match id {
            Identifier::Id(id) => diesel::delete(schools::table.find(id)).execute(conn).await,
            _ => {
                log::error!("Wrong school identifier. Expecting uuid type. Got {:?}", id);
                Err(Error::NotFound)?
            }
        };
        match number_deleted {
            Ok(num) => Ok(num),
            Err(e) => {
                log::error!("Failed to delete school: {}", e);
                Err(e)
            }
        }
    }
}
