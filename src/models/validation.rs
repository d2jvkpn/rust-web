use once_cell::sync::Lazy;
use regex::Regex;
use validator::validate_email; // validate_phone

// alphanumeric(62) + special characters(10)
pub const PASSWORD_CHARS: [&str; 4] = ["0-9", "a-z", "A-Z", "!@#$%^&*()"];
pub const PASSWORD_RANGE: [u8; 2] = [8, 32];

static RE_DATE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\d{4,}-\d{2}-\d{2}$").unwrap());

pub(crate) fn valid_phone(v: &str) -> Result<(), &str> {
    if v.len() > 20 {
        return Err("the length of phone excceds 20");
    }
    // if !validate_phone(v) {
    //     return Err("invalid phone numer");
    // }

    Ok(())
}

pub(crate) fn valid_email(v: &str) -> Result<(), &str> {
    if v.len() > 128 {
        return Err("the length of email excceds 128");
    }
    if !validate_email(v) {
        return Err("email contains forbidden characters");
    }

    Ok(())
}

pub(crate) fn valid_name(v: &str) -> Result<(), &str> {
    if v.is_empty() {
        return Err("name is empty");
    } else if v.len() > 32 {
        return Err("the length of name exceeds 32");
    }
    Ok(())
}

pub(crate) fn valid_birthday(v: &str) -> Result<(), &str> {
    if !RE_DATE.is_match(v) {
        Err("invalid birthday")
    } else {
        Ok(())
    }
}

// TODO: using regexp
pub(crate) fn valid_password(password: &str) -> Result<(), &str> {
    if password.len() < 8 {
        return Err("the length of password is less than 8");
    }
    if password.len() > 32 {
        return Err("the length of password exceeds 32");
    }

    let special_chars = "!@#$%^&*()";

    let (mut digits, mut specials) = (0, 0);
    let (mut lowers, mut uppers) = (0, 0);

    for c in password.chars() {
        match c {
            c if c.is_ascii_digit() => digits += 1,
            c if c.is_lowercase() => lowers += 1,
            c if c.is_uppercase() => uppers += 1,
            c if special_chars.contains(c) => specials += 1,
            _ => return Err("password contains invalid chars"),
        }
    }

    if digits * lowers * uppers * specials > 0 {
        Ok(())
    } else {
        Err("password must contains digits, lowercases, uppercase and !@#$%^&*")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_valid_password() {
        assert!(valid_password("1aA00000@x").is_ok());
        assert!(valid_password("1aA00000&").is_ok());
    }
}
