use std::fmt::Debug;

use chrono::NaiveDateTime;
use diesel::result::Error;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::helper::enums::Identifier;
use crate::helper::exceptions::RepositoryError;
use crate::interfaces::repository_interface::IRepository;
use crate::models::user_models::UserModel;
use crate::schema::users;

// fn get_user_statement(record_id: &Identifier)  {
//     let user_statement = match record_id {
//         Identifier::Id(_id) => {
//             users.filter(id.eq(_id))
//         }
//         Identifier::email(_email) => {
//             users.filter(email.eq(_email))
//         }
//     };
//     user_statement
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserCreate {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserUpdate {
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRepository;

impl IRepository<'_, UserCreate, UserUpdate, User> for UserRepository {
    type Model = UserModel;
    async fn create(conn: &mut AsyncPgConnection, data: UserCreate) -> Result<User, Error> {
        let new_user = Self::Model {
            id: Uuid::new_v4(),
            email: data.email,
            first_name: data.first_name,
            last_name: data.last_name,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: None,
        };
        // create user
        let created_user = diesel::insert_into(crate::schema::users::table)
            .values(&new_user)
            .get_result::<Self::Model>(conn)
            .await;
        match created_user {
            Err(e) => {
                log::error!("Failed to create user: {}", e);
                Err(e)
            }
            Ok(created_user) => Ok(User {
                id: created_user.id,
                email: created_user.email,
                first_name: created_user.first_name,
                last_name: created_user.last_name,
                created_at: created_user.created_at,
                updated_at: created_user.updated_at,
            }),
        }
    }

    async fn get(conn: &mut AsyncPgConnection, id: &Identifier) -> Result<Option<User>, Error> {
        let user = match id {
            Identifier::Id(_id) => users::table
                .find(_id)
                .get_result::<Self::Model>(conn)
                .await
                .map(Some),
            Identifier::Email(_email) => users::table
                .filter(users::email.eq(_email))
                .get_result::<Self::Model>(conn)
                .await
                .map(Some),
        };

        match user {
            Ok(None) => {
                log::warn!("User not found for user {:?}", id);
                Ok(None)
            }
            Ok(Some(user)) => Ok(Some(User {
                id: user.id,
                email: user.email,
                first_name: user.first_name,
                last_name: user.last_name,
                created_at: user.created_at,
                updated_at: user.updated_at,
            })),
            Err(e) => {
                log::error!("Failed to get user: {}", e);
                Err(e)
            }
        }
    }

    async fn update(
        conn: &mut AsyncPgConnection,
        id: &Identifier,
        new_data: UserUpdate,
    ) -> Result<User, Error> {
        let user = match id {
            Identifier::Id(_id) => {
                diesel::update(users::table.find(_id))
                    .set((
                        users::first_name.eq(new_data.first_name),
                        users::last_name.eq(new_data.last_name),
                        users::updated_at.eq(chrono::Utc::now().naive_utc()),
                    ))
                    .get_result::<Self::Model>(conn)
                    .await
            }
            Identifier::Email(_email) => {
                diesel::update(users::table.filter(users::email.eq(_email)))
                    .set((
                        users::first_name.eq(new_data.first_name),
                        users::last_name.eq(new_data.last_name),
                        users::updated_at.eq(chrono::Utc::now().naive_utc()),
                    ))
                    .get_result::<Self::Model>(conn)
                    .await
            }
        };
        match user {
            Ok(user) => {
                log::info!("User {:?} updated", user.id);
                Ok(User {
                    id: user.id,
                    email: user.email,
                    first_name: user.first_name,
                    last_name: user.last_name,
                    created_at: user.created_at,
                    updated_at: user.updated_at,
                })
            }
            Err(e) => {
                log::error!("Failed to update user: {}", e);
                Err(e)
            }
        }
    }

    async fn delete(conn: &mut AsyncPgConnection, id: &Identifier) -> Result<usize, Error> {
        let num_deleted_row = match id {
            Identifier::Id(_id) => diesel::delete(users::table.find(_id)).execute(conn).await,
            Identifier::Email(_email) => {
                diesel::delete(users::table.filter(users::email.eq(_email)))
                    .execute(conn)
                    .await
            }
        };

        match num_deleted_row {
            Ok(num_deleted_row) => Ok(num_deleted_row),
            Err(e) => {
                log::error!("Failed to delete user: {}", e);
                Err(e)
            }
        }
    }
}
