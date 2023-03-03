use super::configuration::{Configuration, Jwt};
use crate::middlewares::response::Error;
use actix_web::http::header::{self, HeaderName};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};

pub struct Config(Configuration);
static CONFIG_INSTANCE: OnceCell<Config> = OnceCell::new();

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct JwtPayload {
    pub user_id: i32,
    pub exp: i64,
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
    pub fn jwt_header() -> HeaderName {
        header::AUTHORIZATION
    }

    pub fn jwt_sign(mut data: JwtPayload) -> Result<String, Error> {
        let jwt = Config::get_jwt().ok_or(Error::Internal("jwt is unset".into()))?;
        data.exp = (Utc::now() + Duration::minutes(jwt.alive_mins as i64)).timestamp();

        let key = EncodingKey::from_secret(jwt.key.as_ref());
        let token =
            encode(&Header::default(), &data, &key).map_err(|e| Error::Internal(e.to_string()))?;

        Ok("Bearer ".to_owned() + &token)
    }

    pub fn jwt_verify(token: String) -> Result<JwtPayload, Error> {
        if !token.starts_with("Bearer ") {
            return Err(Error::Unauthenticated("invalid token 1".to_string()));
        }

        let jwt = Config::get_jwt().ok_or(Error::Internal("jwt is unset".into()))?;
        let key = DecodingKey::from_secret(jwt.key.as_ref());

        // TokenData<JwtPayload> { header, claims }
        let data = decode::<JwtPayload>(&token[7..], &key, &Validation::default())
            .map_err(|_| Error::Unauthenticated("invalid token 2".to_string()))?;

        Ok(data.claims)
    }
}
