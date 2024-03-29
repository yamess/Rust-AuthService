use chrono::Duration;
use diesel::result::Error;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::configs::common::AuthConfig;

use crate::models::user_model::UserModel;
use crate::schema::users;
use crate::schemas::auth_schemas::{LoginRequest, LoginResponse, TokenClaims};
use crate::services::password_service::PasswordService;
use crate::services::token_service::TokenService;

pub struct AuthService;

impl AuthService {
    pub async fn login(
        conn: &mut AsyncPgConnection,
        login_request: LoginRequest,
        auth_config: &AuthConfig,
    ) -> Result<LoginResponse, Error> {
        let user = users::table
            .filter(users::email.eq(&login_request.email))
            .get_result::<UserModel>(conn)
            .await;

        match user {
            Ok(_user) => {
                log::info!("User found: {}", &login_request.email);
                if !PasswordService::verify(&login_request.password.to_string(), &_user.password) {
                    log::error!("Wrong credentials for user {}", &login_request.email);
                    return Err(Error::NotFound);
                }
                let creation_time = chrono::Utc::now().timestamp();
                let expiration_time = chrono::Utc::now().timestamp()
                    + Duration::minutes(auth_config.token_expire_minutes).num_seconds();

                let _token = TokenService::encode(
                    &auth_config.secret_key,
                    TokenClaims {
                        exp: expiration_time,
                        iat: creation_time,
                        sub: _user.id,
                        email: _user.email,
                        tenant_id: None, // @TODO: Replace when Student table implemented
                        admin: _user.is_admin,
                        active: _user.is_active,
                    },
                )
                .await;

                match _token {
                    Err(_e) => {
                        log::error!("Failed to encode payload: {}", _e);
                        return Err(Error::NotFound);
                    }
                    Ok(tok) => Ok(LoginResponse { token: tok }),
                }
            }
            Err(_e) => {
                log::error!("Failed to get user: {}", _e);
                return Err(_e);
            }
        }
    }
}
