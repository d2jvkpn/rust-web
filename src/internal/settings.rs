use super::configuration::Configuration;
use crate::{db::db_token::check_token_in_table, middlewares::Error, models::token::JwtPayload};
use actix_web::{dev::Payload, http::header::AUTHORIZATION, FromRequest, HttpMessage, HttpRequest};
use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine as _};
use chrono::{Duration, Utc};
use futures::executor::block_on;
use jsonwebtoken::{
    decode, encode, errors::ErrorKind::ExpiredSignature, DecodingKey, EncodingKey, Header,
    Validation,
};
use once_cell::sync::OnceCell;
use sha2::{Digest, Sha256};
use sqlx::PgPool;
use std::future::{ready, Ready};

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
    pub fn pool() -> Option<&'static PgPool> {
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

    pub fn jwt_sign(data: &mut JwtPayload) -> Result<String, Error> {
        let (jwt_key, alive_mins) = Self::jwt().ok_or(Error::unexpected_error1())?;

        let now = Utc::now();
        data.iat = now.timestamp();
        data.exp = (now + Duration::minutes(alive_mins as i64)).timestamp();

        let key = EncodingKey::from_secret(jwt_key);

        let token =
            encode(&Header::default(), &data, &key).map_err(|_| Error::unexpected_error1())?;

        Ok("Bearer ".to_owned() + &token)
    }

    pub fn jwt_verify(req: &HttpRequest) -> Result<JwtPayload, Error> {
        // let (jwt_key, _) = Self::jwt().ok_or(Error::Internal("configuration is unset".into()))?;
        let settings = OC_SETTINGS.get().ok_or(Error::unexpected_error1())?;
        let jwt_key = &settings.jwt_key;

        let key = DecodingKey::from_secret(jwt_key);
        let prefix = "Bearer ";

        let msg = "not logged in, please provide token".to_string();
        let value = req.headers().get(AUTHORIZATION).ok_or(Error::unauthenticated(msg))?;

        let msg = "failed to parse token".to_string();
        let token = value.to_str().map_err(|_| Error::unauthenticated(msg))?;

        if !token.starts_with(prefix) {
            return Err(Error::unauthenticated("invalid token format".to_string()));
        }
        // TokenData<JwtPayload>: TokenData{ header, claims }
        let data = decode::<JwtPayload>(&token[prefix.len()..], &key, &Validation::default())
            .map_err(|e| {
                let em = if e.kind() == &ExpiredSignature {
                    "token expired"
                } else {
                    "failed to decode token"
                };
                Error::unauthenticated(em.to_string())
            })?;
        // if data.claims.iat > Utc::now().timestamp() {
        //    return Err(Error::Unauthenticated("token expired".into()));
        // }

        let data = data.claims; // JwtPayload

        // ?? a blocking task
        block_on(check_token_in_table(&settings.pool, data.token_id))?;

        Ok(data)
    }
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

// works only after Config.jwt_verify, but it can't be call in Box::pin(async move {}
#[allow(dead_code)]
pub fn user_id_from_exts(req: &HttpRequest) -> Option<i32> {
    let exts = req.extensions();
    let data = exts.get::<JwtPayload>()?;
    Some(data.user_id)
}

// !! hasn't verify token(Config::jwt_verify) yet
#[allow(dead_code)]
pub fn user_id_from_header(req: &HttpRequest) -> Option<i32> {
    let prefix = "Bearer ";
    let value = req.headers().get(AUTHORIZATION)?;

    let mut token = value.to_str().ok()?;
    if !token.starts_with(prefix) {
        return None;
    }
    token = &token[prefix.len()..];

    let fields = token.split('.').collect::<Vec<&str>>();
    let payload_str = fields.get(1)?;
    let bytes = STANDARD_NO_PAD.decode(payload_str).ok()?;
    let data: JwtPayload = serde_json::from_slice(&bytes).ok()?;

    Some(data.user_id)
}

#[cfg(test)]
mod tests {
    use hex_literal::hex;
    use jsonwebtoken::decode_header;
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

    #[test]
    fn t_decode_header() {
        let header = decode_header("eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpYXQiOjE2NzgxNjI4MTEsImV4cCI6MTY3ODE2NDYxMSwidG9rZW5JZCI6Ijk1OGMyNmRiLTYxOGEtNDM1MC1hNTQ2LWM4NTRjYmEwYTZiYiIsInVzZXJJZCI6MSwicm9sZSI6ImFkbWluIiwicGxhdGZvcm0iOiJ1bmtub3duIn0.ePyZkG91NKmeV95-a_3jFcWzsnxtxTsXfdcllcSkQIM");

        println!("~~~ header: {:?}", header);
    }
}
