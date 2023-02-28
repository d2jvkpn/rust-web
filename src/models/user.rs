use super::validation::*;
use crate::utils::{update_option, update_value};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::Type)]
#[serde(rename_all = "camelCase")]
#[sqlx(type_name = "user_status", rename_all = "lowercase")] // type_name: enum type name in postgres, rename_all = "snake_case"
pub enum Status {
    OK,
    Blocked,
    Deleted,
}

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::Type)]
#[serde(rename_all = "camelCase")]
#[sqlx(type_name = "user_role", rename_all = "lowercase")] // type_name: enum type name in postgres, rename_all = "snake_case"
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

    pub fn update(&mut self, mut item: UpdateUser) -> Result<bool, &str> {
        let updated = update_value(&mut self.name, &mut item.name)
            || update_option(&mut self.birthday, &mut item.birthday);

        Ok(updated)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct CreateUser {
    pub phone: Option<String>,
    pub email: Option<String>,
    pub name: String,
    pub birthday: Option<String>,
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

        Ok(())
    }
}

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
