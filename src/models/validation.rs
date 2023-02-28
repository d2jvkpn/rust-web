use once_cell::sync::Lazy;
use regex::Regex;
use validator::{validate_email, validate_phone};

static RE_DATE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap());

pub fn valid_phone(v: &str) -> Result<(), &str> {
    if v.len() > 20 {
        return Err("the length of phone excceds 20");
    }
    if !validate_phone(v) {
        return Err("invalid phone numer");
    }

    Ok(())
}

pub fn valid_email(v: &str) -> Result<(), &str> {
    if v.len() > 128 {
        return Err("the length of email excceds 20");
    }
    if !validate_email(v) {
        return Err("email contains forbidden characters".into());
    }

    Ok(())
}

pub fn valid_name(v: &str) -> Result<(), &str> {
    if v.is_empty() {
        return Err("name is empty");
    } else if v.len() > 32 {
        return Err("the length of name exceeds 32");
    }
    Ok(())
}

pub fn valid_birthday(v: &str) -> Result<(), &str> {
    if !RE_DATE.is_match(&v) {
        return Err("invalid birthday");
    }

    return Ok(());
}
