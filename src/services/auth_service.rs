use std::collections::HashSet;

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
                if !verify_password(&login_request.password, &_user.password) {
                    log::error!("Wrong credentials for user {}", &login_request.email);
                    return Err(Error::NotFound);
                }
                let creation_time = chrono::Utc::now().timestamp();
                let expiration_time = chrono::Utc::now().timestamp()
                    + Duration::minutes(auth_config.token_expire_minutes).num_seconds();
                let token = Self::generate_token(
                    &auth_config.secret_key,
                    TokenClaims {
                        aud: Some(auth_config.to_owned().audience),
                        exp: expiration_time,
                        iat: creation_time,
                        iss: auth_config.to_owned().issuer,
                        nbf: creation_time,
                        sub: _user.id.to_string(),
                        email: _user.email,
                    },
                )
                    .await;
                Ok(LoginResponse { token })
            }
            Err(e) => {
                log::error!("Failed to get user: {}", e);
                return Err(e);
            }
        }
    }

    async fn generate_token(secret: &str, claim: TokenClaims) -> String {
        let token = encode(
            &Header::default(),
            &claim,
            &EncodingKey::from_secret(secret.as_ref()),
        )
            .map_err(|e| {
                log::error!("Failed to generate token: {}", e);
                e
            })
            .unwrap();
        token
    }

    pub async fn decode_token(
        token: &str,
        auth_config: &AuthConfig,
    ) -> Result<TokenClaims, jsonwebtoken::errors::Error> {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_audience(&[auth_config.audience.to_owned()]);
        validation.set_issuer(&[auth_config.issuer.to_owned()]);
        validation.validate_aud = true;
        validation.validate_nbf = true;
        validation.validate_exp = true;

        let token_data = decode::<TokenClaims>(
            token,
            &DecodingKey::from_secret(&auth_config.secret_key.as_ref()),
            &validation,
        );
        token_data.map(|data| data.claims
        ).map_err(|e| {
            log::error!("Failed to authenticate token: {}", e);
            e
        })
    }
}
