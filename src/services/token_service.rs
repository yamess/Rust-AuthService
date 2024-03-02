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

    /**
     * Decodes a token with the given secret and claim
     *
     * @param token: &str
     * @param auth_config: &AuthConfig
     */
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

#[cfg(test)]
mod tests {
    use chrono::{Duration, Utc};
    use uuid::Uuid;

    use crate::configs::common::AuthConfig;
    use crate::schemas::auth_schemas::TokenClaims;

    use super::*;

    const SECRET_KEY: &str = "PEP+DnYqfglRX+vextkRcA=";

    #[tokio::test]
    async fn test_encode() {
        let auth_config = AuthConfig {
            secret_key: SECRET_KEY.to_string(),
            token_expire_minutes: 10_i64,
            issuer: "".to_string(),
            audience: "".to_string(),
        };
        let token_claims = TokenClaims {
            exp: Utc::now().timestamp()
                + Duration::minutes(auth_config.token_expire_minutes).num_seconds(),
            iat: Utc::now().timestamp(),
            sub: Uuid::parse_str("70819fbb-e89c-454a-b80a-507c994264ee").unwrap(),
            email: "test@domain.com".to_string(),
            tenant_id: None,
            admin: false,
            active: true,
        };
        let token = TokenService::encode(&auth_config.secret_key, token_claims).await;
        assert!(token.is_ok());
    }

    #[tokio::test]
    async fn test_decode() {
        let auth_config = AuthConfig {
            secret_key: SECRET_KEY.to_string(),
            token_expire_minutes: 10_i64,
            issuer: "".to_string(),
            audience: "".to_string(),
        };
        let token_claims = TokenClaims {
            exp: Utc::now().timestamp()
                + Duration::minutes(auth_config.token_expire_minutes).num_seconds(),
            iat: Utc::now().timestamp(),
            sub: Uuid::parse_str("70819fbb-e89c-454a-b80a-507c994264ee").unwrap(),
            email: "test@domain.com".to_string(),
            tenant_id: None,
            admin: false,
            active: true,
        };
        let token = TokenService::encode(&auth_config.secret_key, token_claims).await;
        assert!(token.is_ok());
        let token = token.unwrap();
        let decoded = TokenService::decode(&token, &auth_config).await;
        assert!(&decoded.is_ok());

        let claims = decoded.unwrap();
        assert_eq!(claims.email == "test@domain.com", true);
        assert_eq!(claims.admin, false);
        assert_eq!(claims.active, true);
        assert_eq!(claims.tenant_id, None);
        assert_eq!(
            claims.sub,
            Uuid::parse_str("70819fbb-e89c-454a-b80a-507c994264ee").unwrap()
        );
        assert_eq!(claims.exp > Utc::now().timestamp(), true);
        assert_eq!(claims.iat <= Utc::now().timestamp(), true);
    }
}
