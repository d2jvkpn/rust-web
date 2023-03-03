use super::configuration::{Configuration, Jwt};
use crate::{middlewares::response::Error, models::user::Role};
use actix_web::{dev::Payload, http, FromRequest, HttpMessage, HttpRequest};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::future::{ready, Ready};

pub struct Config(Configuration);
static CONFIG_INSTANCE: OnceCell<Config> = OnceCell::new();

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JwtPayload {
    // pub iss: String, // issuer
    // pub sub: String, // subject
    pub user_id: i32,
    pub iat: i64, // issued at
    pub exp: i64, // expiry
    pub role: Role,
}

impl Config {
    pub fn set(configuration: Configuration) -> Result<(), &'static str> {
        // convert Result<(), Config>
        CONFIG_INSTANCE.set(Self(configuration)).map_err(|_| "can't set configuration")
    }

    fn get() -> Option<&'static Config> {
        CONFIG_INSTANCE.get()
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

    pub fn jwt_verify(token: String) -> Result<JwtPayload, Error> {
        if !token.starts_with("Bearer ") {
            return Err(Error::Unauthenticated("b1::invalid token".to_string()));
        }

        let jwt = Config::get_jwt().ok_or(Error::Internal("jwt is unset".into()))?;
        let key = DecodingKey::from_secret(jwt.key.as_ref());

        // TokenData<JwtPayload> { header, claims }
        let data = decode::<JwtPayload>(&token[7..], &key, &Validation::default())
            .map_err(|_| Error::Unauthenticated("b2::invalid token".to_string()))?;

        Ok(data.claims)
    }
}

impl FromRequest for JwtPayload {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let msg = "a1::you are not logged in, please provide token".to_string();
        let value = match req.headers().get(http::header::AUTHORIZATION) {
            Some(v) => v,
            None => return ready(Err(Error::Unauthenticated(msg))),
        };

        let msg = "a2::invalid token".to_string();
        let token = match value.to_str() {
            Ok(v) => v,
            Err(_) => return ready(Err(Error::Unauthenticated(msg))),
        };

        let payload = match Config::jwt_verify(token.to_string()) {
            Ok(v) => v,
            Err(e) => return ready(Err(e)),
        };

        if payload.iat > Utc::now().timestamp() {
            return ready(Err(Error::Unauthenticated("a3::token expired".into())));
        }

        req.extensions_mut().insert(payload.clone());
        ready(Ok(payload))
    }
}
