use std::fmt::Debug;

use diesel::result::Error;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::helper::enums::Identifier;
use crate::interfaces::repository_interface::IRepository;
use crate::models::user_model::UserModel;
use crate::schema::users;
use crate::schemas::user_schemas::{UserCreate, UserResponse, UserUpdate};
use crate::services::password_service::PasswordService;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRepository;

impl IRepository<'_, UserCreate, UserUpdate, UserResponse> for UserRepository {
    type Model = UserModel;

    async fn create(conn: &mut AsyncPgConnection, data: UserCreate) -> Result<UserResponse, Error> {
        if !PasswordService::validate(&data.password) {
            log::error!("Password length must be at least 8 characters");
            // @TODO: Replace with custom error
            return Err(Error::DatabaseError(
                diesel::result::DatabaseErrorKind::CheckViolation,
                Box::new("Password length must be at least 8 characters".to_string()),
            ));
        }
        let hashed_password = PasswordService::hash(&data.password);

        let new_user = Self::Model::new(data.email, hashed_password, data.is_active, data.is_admin);
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
            Ok(created_user) => Ok(UserResponse {
                id: created_user.id,
                email: created_user.email,
                is_active: created_user.is_active,
                is_admin: created_user.is_admin,
                created_at: created_user.created_at,
                updated_at: created_user.updated_at,
            }),
        }
    }

    async fn get(
        conn: &mut AsyncPgConnection,
        id: &Identifier,
    ) -> Result<Option<UserResponse>, Error> {
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
            Ok(Some(user)) => Ok(Some(UserResponse {
                id: user.id,
                email: user.email,
                is_active: user.is_active,
                is_admin: user.is_admin,
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
    ) -> Result<UserResponse, Error> {
        let old_data = match id {
            Identifier::Id(id) => {
                users::table
                    .find(id)
                    .get_result::<Self::Model>(conn)
                    .await?
            }
            Identifier::Email(_email) => {
                users::table
                    .filter(users::email.eq(_email))
                    .get_result::<Self::Model>(conn)
                    .await?
            }
        };

        let user = diesel::update(&old_data)
            .set((
                users::is_active.eq(new_data.is_active),
                users::is_admin.eq(new_data.is_admin),
                users::updated_at.eq(chrono::Utc::now().naive_utc()),
            ))
            .get_result::<Self::Model>(conn)
            .await;

        match user {
            Ok(user) => {
                log::info!("User {:?} password updated successfully", user.id);
                Ok(UserResponse {
                    id: user.id,
                    email: user.email,
                    is_active: user.is_active,
                    is_admin: user.is_admin,
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
