use validator::{validate_email, validate_phone};

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
