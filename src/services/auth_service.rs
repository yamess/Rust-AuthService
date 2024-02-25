use bcrypt;
use chrono::Duration;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::result::Error;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use jsonwebtoken::{Algorithm, decode, DecodingKey, encode, EncodingKey, Header, Validation};

use crate::configs::common::AuthConfig;
use crate::helper::enums::Identifier;
use crate::helper::utils::verify_password;
use crate::interfaces::repository_interface::IRepository;
use crate::models::user_models::UserModel;
use crate::repositories::user_repository::UserRepository;
use crate::schema::users;
use crate::schemas::auth_schema::{LoginRequest, LoginResponse, TokenClaims};

pub struct AuthService;

impl AuthService {
    pub async fn login(
        conn: &mut AsyncPgConnection,
        email: String,
        password: String,
        auth_config: AuthConfig,
    ) -> Result<LoginResponse, Error> {
        let user = users::table
            .filter(users::email.eq(&email))
            .get_result::<UserModel>(conn)
            .await;

        match user {
            Ok(_user) => {
                if !verify_password(&password, &_user.password) {
                    log::error!("Wrong credentials for user {}", &email);
                    return Err(Error::NotFound);
                }
                let creation_time = chrono::Utc::now().timestamp();
                let expiration_time = chrono::Utc::now().timestamp() + Duration::minutes(auth_config.token_expire_minutes).num_seconds();
                let token = Self::generate_token(
                    auth_config.secret_key,
                    TokenClaims {
                        aud: auth_config.audience,
                        exp: expiration_time,
                        iat: creation_time,
                        iss: auth_config.issuer,
                        nbf: creation_time,
                        sub: _user.id.to_string(),
                        email: _user.email,
                    },
                ).await;
                Ok(LoginResponse { token })
            }
            Err(e) => {
                log::error!("Failed to get user: {}", e);
                return Err(e);
            }
        }
    }

    async fn generate_token(secret: String, claim: TokenClaims) -> String {
        let token = encode(
            &Header::default(),
            &claim,
            &EncodingKey::from_secret(secret.as_ref()),
        ).map_err(|e| {
            log::error!("Failed to generate token: {}", e);
            e
        }).unwrap();
        token
    }
}
