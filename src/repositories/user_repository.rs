use std::fmt::Debug;

use chrono::NaiveDateTime;
use diesel::result::Error;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::helper::exceptions::RepositoryError;
use crate::interfaces::repository_interface::IRepository;
use crate::models::user_models::UserModel;

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

impl IRepository<'_, UserCreate, User> for UserRepository {
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
}
// async fn get(conn: &mut AsyncPgConnection, id: &Identifier) -> Result<Option<User>, Error> {
//     // get user
//     let user = get_user_statement(id)
//         .get_result::<Self::Model>(conn)
//         .await
//         .map_err(|e| {
//             log::error!("Failed to get user: {}", e);
//             e
//         })
//         .ok();
//
//     Ok(match user {
//         None => {
//             log::warn!("User not found for email {:?}", id);
//             return Ok(None);
//         }
//         Some(user) => Some(User {
//             id: user.id,
//             email: user.email,
//             password: user.password,
//             created_at: user.created_at,
//             updated_at: user.updated_at,
//         }),
//     })
// }
//
// async fn update(conn: &mut AsyncPgConnection, id: &Identifier, user: User) -> Result<User,
//     Error> {
//     // update user
//     let old_data = Self::get(conn, id.clone())
//         .await
//         .unwrap()
//         .ok_or("User not found")
//         .unwrap();
//     let new_data = Self::Model {
//         id: old_data.id,
//         email: old_data.email,
//         password: user.password,
//         created_at: old_data.created_at,
//         updated_at: Some(chrono::Utc::now().naive_utc()),
//     };
//     let updated_user = diesel::update(get_user_statement(id))
//         .set(&new_data)
//         .get_result::<Self::Model>(conn)
//         .await
//         .map_err(|e| {
//             log::error!("Failed to update user: {}", e);
//             e
//         }).Ok();
//     Ok(new_data)
// }
// async fn delete(conn: &mut AsyncPgConnection, id: &Identifier) -> Result<usize, Error> {
//     // delete user
//     let num_deleted_row = diesel::delete(get_user_statement(id))
//         .execute(conn)
//         .await
//         .map_err(|e| {
//             log::error!("Failed to delete user: {}", e);
//             e
//         })
//         .unwrap_or(0);
//     Ok(num_deleted_row)
// }
// }
