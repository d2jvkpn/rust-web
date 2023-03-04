use super::configuration::{Configuration, Jwt};
use crate::{middlewares::response::Error, models::user::Role};
use actix_web::{dev::Payload, http::header::AUTHORIZATION, FromRequest, HttpMessage, HttpRequest};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::future::{ready, Ready};

pub struct Config(Configuration);
static OC_CONFIG: OnceCell<Config> = OnceCell::new();

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

impl Config {
    pub fn set(configuration: Configuration) -> Result<(), &'static str> {
        // convert Result<(), Config>
        OC_CONFIG.set(Self(configuration)).map_err(|_| "can't set configuration")
    }

    fn get() -> Option<&'static Config> {
        OC_CONFIG.get()
    }

    fn get_jwt() -> Option<&'static Jwt> {
        let config = Self::get()?;
        Some(&config.0.jwt)
    }

    // pub fn jwt_header() -> &'static str {
    //    "authorization"
    // }
    // pub fn jwt_header() -> HeaderName {
    //     header::AUTHORIZATION
    // }

    pub fn jwt_sign(mut data: JwtPayload) -> Result<String, Error> {
        let jwt = Config::get_jwt().ok_or(Error::Internal("jwt is unset".into()))?;

        let now = Utc::now();
        data.iat = now.timestamp();
        data.exp = (now + Duration::minutes(jwt.alive_mins as i64)).timestamp();

        let key = EncodingKey::from_secret(jwt.key.as_ref());

        let token =
            encode(&Header::default(), &data, &key).map_err(|e| Error::Internal(e.to_string()))?;

        Ok("Bearer ".to_owned() + &token)
    }

    pub fn jwt_verify(req: &HttpRequest) -> Result<JwtPayload, Error> {
        let jwt = Self::get_jwt().ok_or(Error::Internal("jwt is unset".into()))?;
        let key = DecodingKey::from_secret(jwt.key.as_ref());
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

impl FromRequest for JwtPayload {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let jwt_payload = match Config::jwt_verify(req) {
            Ok(v) => v,
            Err(e) => return ready(Err(e)),
        };

        req.extensions_mut().insert(jwt_payload.clone());
        ready(Ok(jwt_payload))
    }
}
