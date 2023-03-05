use super::configuration::Configuration;
use crate::{middlewares::response::Error, models::user::Role};
use actix_web::{dev::Payload, http::header::AUTHORIZATION, FromRequest, HttpMessage, HttpRequest};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::future::{ready, Ready};

#[allow(dead_code)]
pub struct Settings {
    configuration: Configuration,
    pool: PgPool,
}

static OC_SETTINGS: OnceCell<Settings> = OnceCell::new();

#[allow(dead_code)]
impl Settings {
    pub fn set(configuration: Configuration, pool: PgPool) -> Result<(), &'static str> {
        // convert Result<(), Config>
        OC_SETTINGS.set(Self { configuration, pool }).map_err(|_| "can't set configuration")
    }

    fn pool() -> Option<&'static PgPool> {
        let settings = OC_SETTINGS.get()?;
        Some(&settings.pool)
    }

    fn configuration() -> Option<&'static Configuration> {
        let settings = OC_SETTINGS.get()?;
        Some(&settings.configuration)
    }

    // pub fn jwt_header() -> &'static str {
    //    "authorization"
    // }
    // pub fn jwt_header() -> HeaderName {
    //     header::AUTHORIZATION
    // }

    pub fn jwt_sign(mut data: JwtPayload) -> Result<String, actix_web::Error> {
        let config =
            Settings::configuration().ok_or(Error::Internal("configuration is unset".into()))?;

        let now = Utc::now();
        data.iat = now.timestamp();
        data.exp = (now + Duration::minutes(config.jwt.alive_mins as i64)).timestamp();

        let key = EncodingKey::from_secret(config.jwt.key.as_ref());

        let token =
            encode(&Header::default(), &data, &key).map_err(|e| Error::Internal(e.to_string()))?;

        Ok("Bearer ".to_owned() + &token)
    }

    pub fn jwt_verify(req: &HttpRequest) -> Result<JwtPayload, Error> {
        let config =
            Self::configuration().ok_or(Error::Internal("Configuration is unset".into()))?;

        let key = DecodingKey::from_secret(config.jwt.key.as_ref());
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
