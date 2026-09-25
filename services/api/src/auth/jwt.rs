use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode, get_current_timestamp,
};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Clone)]
pub struct JwtService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    issuer: String,
    audience: String,
    lifetime: Duration,
    key_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AccessClaims {
    pub iss: String,
    pub aud: String,
    pub sub: String,
    pub sid: String,
    pub jti: String,
    pub iat: u64,
    pub nbf: u64,
    pub exp: u64,
    pub ver: u8,
}

pub struct IssuedAccessToken {
    pub token: String,
    pub expires_at: OffsetDateTime,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AuthContext {
    pub account_id: Uuid,
    pub session_id: Uuid,
}

impl JwtService {
    pub fn new(
        key: &[u8],
        issuer: String,
        audience: String,
        lifetime: Duration,
    ) -> Result<Self, &'static str> {
        if key.len() < 32 || issuer.is_empty() || audience.is_empty() {
            return Err("Invalid access-token configuration.");
        }
        Ok(Self {
            encoding_key: EncodingKey::from_secret(key),
            decoding_key: DecodingKey::from_secret(key),
            issuer,
            audience,
            lifetime,
            key_id: "tro-hs256-v1".to_owned(),
        })
    }

    pub fn issue(
        &self,
        account_id: Uuid,
        session_id: Uuid,
    ) -> Result<IssuedAccessToken, jsonwebtoken::errors::Error> {
        let now = get_current_timestamp();
        let expires = now + self.lifetime.as_secs();
        let claims = AccessClaims {
            iss: self.issuer.clone(),
            aud: self.audience.clone(),
            sub: account_id.to_string(),
            sid: session_id.to_string(),
            jti: Uuid::new_v4().to_string(),
            iat: now,
            nbf: now.saturating_sub(1),
            exp: expires,
            ver: 1,
        };
        let mut header = Header::new(Algorithm::HS256);
        header.kid = Some(self.key_id.clone());
        Ok(IssuedAccessToken {
            token: encode(&header, &claims, &self.encoding_key)?,
            expires_at: OffsetDateTime::from_unix_timestamp(expires as i64)
                .expect("JWT timestamp is representable"),
        })
    }

    pub fn verify(&self, token: &str) -> Result<AuthContext, jsonwebtoken::errors::Error> {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.leeway = 30;
        validation.validate_nbf = true;
        validation.set_audience(&[&self.audience]);
        validation.set_issuer(&[&self.issuer]);
        validation.set_required_spec_claims(&["exp", "nbf", "iss", "aud", "sub"]);
        let data = decode::<AccessClaims>(token, &self.decoding_key, &validation)?;
        let claims = data.claims;
        let now = get_current_timestamp();
        let structurally_valid = claims.ver == 1
            && claims.iat <= now + 30
            && claims.exp >= claims.iat
            && claims.exp.saturating_sub(claims.iat) <= self.lifetime.as_secs() + 30
            && Uuid::parse_str(&claims.jti).is_ok();
        if !structurally_valid {
            return Err(jsonwebtoken::errors::ErrorKind::InvalidToken.into());
        }
        Ok(AuthContext {
            account_id: Uuid::parse_str(&claims.sub)
                .map_err(|_| jsonwebtoken::errors::ErrorKind::InvalidSubject)?,
            session_id: Uuid::parse_str(&claims.sid)
                .map_err(|_| jsonwebtoken::errors::ErrorKind::InvalidToken)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn service() -> JwtService {
        JwtService::new(
            &[7_u8; 32],
            "https://api.tro.test".to_owned(),
            "tro-desktop-api".to_owned(),
            Duration::from_secs(900),
        )
        .unwrap()
    }

    #[test]
    fn issues_and_verifies_scoped_access_tokens() {
        let account_id = Uuid::new_v4();
        let session_id = Uuid::new_v4();
        let issued = service().issue(account_id, session_id).unwrap();
        assert_eq!(
            service().verify(&issued.token).unwrap(),
            AuthContext {
                account_id,
                session_id,
            }
        );
        assert!(issued.expires_at > OffsetDateTime::now_utc());
    }

    #[test]
    fn rejects_tokens_from_another_key_or_audience() {
        let issued = service().issue(Uuid::new_v4(), Uuid::new_v4()).unwrap();
        let wrong_key = JwtService::new(
            &[8_u8; 32],
            "https://api.tro.test".to_owned(),
            "tro-desktop-api".to_owned(),
            Duration::from_secs(900),
        )
        .unwrap();
        assert!(wrong_key.verify(&issued.token).is_err());
        let wrong_audience = JwtService::new(
            &[7_u8; 32],
            "https://api.tro.test".to_owned(),
            "other".to_owned(),
            Duration::from_secs(900),
        )
        .unwrap();
        assert!(wrong_audience.verify(&issued.token).is_err());
    }
}
