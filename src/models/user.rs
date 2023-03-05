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
    pub phone: Option<String>,
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
pub struct UpdateUserStatus {
    #[serde(default = "User::default_id")]
    pub id: i32,
    pub status: Status,
}

// user login and token
#[derive(Deserialize, Debug, Clone)]
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

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserAndToken {
    pub user: User,
    pub token_name: String,
    pub token_value: String,
}

// update user body
#[derive(Deserialize, Debug, Clone)]
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
