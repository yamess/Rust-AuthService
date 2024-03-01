use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

use crate::configs::common::AuthConfig;
use crate::schemas::auth_schemas::TokenClaims;

pub struct TokenService;

impl TokenService {
    /**
     * Encodes a token with the given secret and claim
     *
     * @param secret: &str
     * @param claim: TokenClaims
     */
    pub async fn encode(
        secret: &str,
        claim: TokenClaims,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        let token = encode(
            &Header::default(),
            &claim,
            &EncodingKey::from_secret(secret.as_ref()),
        )
        .map_err(|e| {
            log::error!("Failed to generate token: {}", e);
            e
        });
        token
    }

    pub async fn decode(
        token: &str,
        auth_config: &AuthConfig,
    ) -> Result<TokenClaims, jsonwebtoken::errors::Error> {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = true;

        let token_data = decode::<TokenClaims>(
            token,
            &DecodingKey::from_secret(&auth_config.secret_key.as_ref()),
            &validation,
        );
        token_data.map(|data| data.claims).map_err(|e| {
            log::error!("Failed to authenticate token: {}", e);
            e
        })
    }
}
