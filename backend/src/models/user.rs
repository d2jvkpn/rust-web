use super::validation::*;
use crate::utils::{update_option, update_value};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// user structure
// sqlx type_name: enum type name in postgres, rename_all = "snake_case"
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, sqlx::Type)]
#[serde(rename_all = "camelCase")]
#[sqlx(type_name = "user_status", rename_all = "lowercase")]
pub enum Status {
    OK,
    Frozen,
    Blocked,
    Deleted,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, sqlx::Type)]
#[serde(rename_all = "camelCase")]
#[sqlx(type_name = "user_role", rename_all = "snake_case")]
pub enum Role {
    Admin,
    Leader,
    Member,
}

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[serde(default = "User::default_id")]
    pub id: i32,
    pub status: Status,
    pub role: Role,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    pub name: String,
    pub birthday: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    fn default_id() -> i32 {
        0
    }

    pub fn update(&mut self, mut item: UpdateUser) -> bool {
        update_value(&mut self.name, &mut item.name)
            || update_option(&mut self.birthday, &mut item.birthday)
    }

    pub fn status_ok(&self) -> Result<(), &str> {
        match self.status {
            Status::OK => Ok(()),
            Status::Frozen => Err("your account is frozen"),
            Status::Blocked => Err("your account is blocked"),
            Status::Deleted => Err("this account is banned from logging in"),
        }
    }
}

// create user query
#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct CreateUser {
    pub phone: Option<String>,
    pub email: Option<String>,
    pub name: String,
    pub birthday: Option<String>,
    pub password: String,
}

impl CreateUser {
    // creation
    pub fn valid(&self) -> Result<(), &str> {
        if self.phone.is_none() && self.email.is_none() {
            return Err("both phone and email are unset");
        }

        if let Some(v) = &self.phone {
            valid_phone(v)?;
        }

        if let Some(v) = &self.email {
            valid_email(v)?;
        }

        valid_name(self.name.as_str())?;

        if let Some(v) = &self.birthday {
            valid_birthday(v)?;
        }

        valid_password(&self.password)?;

        Ok(())
    }
}

// match user query
#[derive(Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct MatchUser {
    pub id: Option<i32>,
    pub phone: Option<String>,
    pub email: Option<String>,
}

impl MatchUser {
    pub fn valid(&self) -> Result<(), &str> {
        if self.id.is_none() && self.phone.is_none() && self.email.is_none() {
            return Err("miss parameter");
        }

        Ok(())
    }
}

// update user status query
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserStatus {
    pub user_id: i32,
    pub status: Status,
}

// update user role query
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserRole {
    pub user_id: i32,
    pub role: Role,
}

// user login and token
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserLogin {
    pub phone: Option<String>,
    pub email: Option<String>,
    pub password: String,
}

impl UserLogin {
    pub fn valid(&self) -> Result<(), &str> {
        if self.phone.is_none() && self.email.is_none() {
            return Err("miss parameter");
        }

        Ok(())
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct UserAndPassword {
    #[sqlx(flatten)]
    pub user: User,
    pub password: String,
}

// update user body
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUser {
    // pub phone: Option<String>,
    // pub email: Option<String>,
    pub name: Option<String>,
    pub birthday: Option<String>,
}
// TODO: validation

impl UpdateUser {
    pub fn valid(&self) -> Result<(), &str> {
        if let Some(v) = &self.name {
            valid_name(v)?;
        }

        if let Some(v) = &self.birthday {
            valid_birthday(v)?;
        }

        Ok(())
    }
}

// update change password
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChangePassword {
    pub old_password: String,
    pub new_password: String,
}

impl ChangePassword {
    pub fn valid(&self) -> Result<(), &str> {
        if self.old_password == self.new_password {
            return Err("the new password is the same as the old password");
        }

        valid_password(&self.old_password)?;
        valid_password(&self.new_password)
    }
}

// reset user password
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResetPassword {
    pub user_id: i32,
    pub new_password: String,
}

impl ResetPassword {
    pub fn valid(&self) -> Result<(), &str> {
        if self.user_id <= 0 {
            return Err("invalid user id");
        }

        valid_password(&self.new_password)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Local, Utc};
    use serde_json::json;

    #[test]
    fn t_serde() {
        let d = ResetPassword { user_id: 42, new_password: "hello".into() };
        let s = json!(d);
        let text = format!("{}", s);
        println!("~~~ {text}");
    }

    #[test]
    fn t_chrono() {
        let now = Utc::now();
        println!("~~~ Utc::now(): {now}, {}", now.with_timezone(&Local));
    }
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Tokens {
    pub access_token: String,
    // pub alive_mins: u32,
    pub access_exp: i64,
    pub refresh_token: String,
    pub refresh_exp: i64,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserAndTokens {
    pub user: User,
    pub tokens: Tokens,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RefreshToken {
    pub refresh_token: String,
}
