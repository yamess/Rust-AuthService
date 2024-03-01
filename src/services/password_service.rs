use diesel::result::Error;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::helper::enums::Identifier;
use crate::models::user_model::UserModel;
use crate::schema::users;
use crate::schemas::user_schemas::PasswordUpdate;

pub struct PasswordService;

impl PasswordService {
    pub fn hash(password: &str) -> String {
        bcrypt::hash(password, 12).unwrap()
    }
    pub fn verify(password: &str, hash: &str) -> bool {
        bcrypt::verify(password, hash).unwrap()
    }
    pub fn validate(password: &str) -> bool {
        (password.len() >= 8) && (password != "password") && (password != "12345678")
    }

    pub async fn update_password(
        conn: &mut AsyncPgConnection,
        id: &Identifier,
        new_data: PasswordUpdate,
    ) -> Result<(), Error> {
        if !Self::validate(&new_data.new_password) {
            log::error!("Password length must be at least 8 characters");
            // @TODO: Replace with custom error
            return Err(Error::DatabaseError(
                diesel::result::DatabaseErrorKind::CheckViolation,
                Box::new("Password length must be at least 8 characters".to_string()),
            ));
        }
        let old_data = match id {
            Identifier::Id(id) => users::table.find(id).get_result::<UserModel>(conn).await?,
            Identifier::Email(email) => {
                users::table
                    .filter(users::email.eq(email))
                    .get_result::<UserModel>(conn)
                    .await?
            }
        };
        if !PasswordService::verify(&new_data.old_password, &old_data.password) {
            log::error!("Wrong credentials for user {}", old_data.email);
            // @TODO: Replace with custom error
            return Err(Error::DatabaseError(
                diesel::result::DatabaseErrorKind::CheckViolation,
                Box::new("Wrong credentials".to_string()),
            ));
        }

        let hashed_password = PasswordService::hash(&new_data.new_password);
        let user = diesel::update(&old_data)
            .set((
                users::password.eq(hashed_password),
                users::updated_at.eq(chrono::Utc::now().naive_utc()),
            ))
            .get_result::<UserModel>(conn)
            .await;

        match user {
            Ok(user) => {
                log::info!("User {:?} password updated successfully", user.id);
                Ok(())
            }
            Err(e) => {
                log::error!("Failed to update user: {}", e);
                Err(e)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_validate_wrong() {
        let wrong_pwd_1 = "password";
        let wrong_pwd_2 = "secrets";
        let wrong_pd_3 = "12345678";
        assert!(!PasswordService::validate(wrong_pwd_1));
        assert!(!PasswordService::validate(wrong_pwd_2));
        assert!(!PasswordService::validate(wrong_pd_3));
    }

    #[tokio::test]
    async fn test_validate_correct() {
        let correct_pwd = "password123";
        assert!(PasswordService::validate(correct_pwd));
    }

    #[tokio::test]
    async fn test_hash() {
        let password = "password123";
        let hashed_password = PasswordService::hash(password);
        assert_ne!(password, hashed_password);
    }

    #[tokio::test]
    async fn test_verify() {
        let password = "password123";
        let hashed_password = PasswordService::hash(password);
        assert!(PasswordService::verify(password, &hashed_password));
    }
}
