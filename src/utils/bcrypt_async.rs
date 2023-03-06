// https://docs.rs/tokio/latest/tokio/task/fn.spawn_blocking.html
use bcrypt::{hash, verify};
use tokio::task;

// P: AsRef<[u8]> -> Result<String, bcrypt::BcryptError>
pub async fn bcrypt_hash(password: String, cost: u32) -> Result<String, String> {
    let result = match task::spawn_blocking(move || hash(password.as_str(), cost)).await {
        Err(e) => return Err(e.to_string()), // task::JoinError
        Ok(v) => v,
    };

    match result {
        Err(e) => Err(e.to_string()), // bcrypt::BcryptError
        Ok(v) => Ok(v),
    }
}

// P: AsRef<[u8]>, &str -> Result<String, bcrypt::BcryptError>
pub async fn bcrypt_verify(password: String, hashed: String) -> Result<bool, String> {
    let result =
        match task::spawn_blocking(move || verify(password.as_str(), hashed.as_str())).await {
            Err(e) => return Err(e.to_string()), // task::JoinError
            Ok(v) => v,
        };

    match result {
        Err(e) => Err(e.to_string()), // bcrypt::BcryptError
        Ok(v) => Ok(v),
    }
}

#[cfg(test)]
mod tests {
    use bcrypt::{hash, verify, DEFAULT_COST};

    #[test]
    fn t_bcrypt() {
        let password = hash("123456", DEFAULT_COST).unwrap();
        let m = verify("123456", &password).unwrap();
        assert!(m);

        let m = verify("123456aaa", &password).unwrap();
        assert!(!m);

        let password = hash("12QWas!@", DEFAULT_COST).unwrap();
        dbg!(&password);
        let m = verify("12QWas!@", &password).unwrap();
        assert!(m);

        let m = verify("12QWas!@", "$2b$12$QnMtKFokkQbxZ8vATa2PU.b2IkTPd8QDumYdgpWsMGNKeX5IOONUW")
            .unwrap();

        assert!(m);
    }
}
