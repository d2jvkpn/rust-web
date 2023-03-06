use super::configuration::Configuration;
use crate::{middlewares::response::Error, models::user::Role};
use actix_web::{dev::Payload, http::header::AUTHORIZATION, FromRequest, HttpMessage, HttpRequest};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sqlx::PgPool;
use std::future::{ready, Ready};
use uuid::Uuid;

pub struct Settings {
    configuration: Configuration,
    pool: PgPool,
    jwt_key: Vec<u8>,
}

static OC_SETTINGS: OnceCell<Settings> = OnceCell::new();

impl Settings {
    pub fn set(configuration: Configuration, pool: PgPool) -> Result<(), &'static str> {
        let mut hasher = Sha256::new();
        hasher.update(configuration.jwt.key.as_bytes());
        let result = hasher.finalize();

        // let strs: Vec<String> = result.iter().map(|b| format!("{:02x}", b)).collect();
        // let jwt_key = strs.join("");

        // convert Result<(), Config>
        OC_SETTINGS
            .set(Self { configuration, pool, jwt_key: result.to_vec() })
            .map_err(|_| "can't set configuration")
    }

    #[allow(dead_code)]
    fn pool() -> Option<&'static PgPool> {
        let settings = OC_SETTINGS.get()?;
        Some(&settings.pool)
    }

    #[allow(dead_code)]
    fn configuration() -> Option<&'static Configuration> {
        let settings = OC_SETTINGS.get()?;
        Some(&settings.configuration)
    }

    // not use settings.configuration.jwt.key as jwt_key
    fn jwt() -> Option<(&'static [u8], u32)> {
        let settings = OC_SETTINGS.get()?;
        // Some((settings.jwt_key.as_bytes(), settings.configuration.jwt.alive_mins))
        Some((&settings.jwt_key, settings.configuration.jwt.alive_mins))
    }

    pub fn jwt_sign(mut data: JwtPayload) -> Result<String, actix_web::Error> {
        let (jwt_key, alive_mins) =
            Self::jwt().ok_or(Error::Internal("configuration is unset".into()))?;

        let now = Utc::now();
        data.iat = now.timestamp();
        data.exp = (now + Duration::minutes(alive_mins as i64)).timestamp();

        let key = EncodingKey::from_secret(jwt_key);

        let token =
            encode(&Header::default(), &data, &key).map_err(|e| Error::Internal(e.to_string()))?;

        Ok("Bearer ".to_owned() + &token)
    }

    pub fn jwt_verify(req: &HttpRequest) -> Result<JwtPayload, Error> {
        let (jwt_key, _) = Self::jwt().ok_or(Error::Internal("configuration is unset".into()))?;

        let key = DecodingKey::from_secret(jwt_key);
        let prefix = "Bearer ";

        let msg = "not logged in, please provide token".to_string();
        let value = req.headers().get(AUTHORIZATION).ok_or(Error::Unauthenticated(msg))?;

        let msg = "failed to parse token".to_string();
        let token = value.to_str().map_err(|_| Error::Unauthenticated(msg))?;

        if !token.starts_with(prefix) {
            return Err(Error::Unauthenticated("invalid token format".to_string()));
        }
        // TokenData<JwtPayload>: TokenData{ header, claims }
        let data = decode::<JwtPayload>(&token[prefix.len()..], &key, &Validation::default())
            .map_err(|_| Error::Unauthenticated("failed to decode token".to_string()))?;

        if data.claims.iat > Utc::now().timestamp() {
            return Err(Error::Unauthenticated("token expired".into()));
        }

        Ok(data.claims)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JwtPayload {
    // pub iss: String, // issuer
    // pub sub: String, // subject
    pub iat: i64, // issued at
    pub exp: i64, // expiry
    pub token_id: Uuid,
    pub user_id: i32,
    pub role: Role,
}

impl FromRequest for JwtPayload {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let jwt_payload = match Settings::jwt_verify(req) {
            Ok(v) => v,
            Err(e) => return ready(Err(e)),
        };

        req.extensions_mut().insert(jwt_payload.clone());
        ready(Ok(jwt_payload))
    }
}

#[cfg(test)]
mod tests {
    use hex_literal::hex;
    use sha2::{Digest, Sha256};

    // echo -n "hello world" | sha256sum | awk '{print $1}'
    #[test]
    fn t_sha2() {
        let mut hasher = Sha256::new();
        hasher.update(b"hello world");

        let result = hasher.finalize();

        let strs: Vec<String> = result.iter().map(|b| format!("{:02x}", b)).collect();
        println!("~~~ {}", strs.join(""));

        assert_eq!(
            result[..],
            hex!("b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9")[..],
        );
    }
}
