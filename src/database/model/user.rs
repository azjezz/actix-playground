use diesel::{deserialize::Queryable, prelude::Insertable};
use md5;
use serde::{Deserialize, Serialize};

use crate::database::schema::users;

pub const USER_FLAG_ADMIN: i32 = 1;
pub const USER_FLAG_SUSPENDED: i32 = 2;
pub const USER_FLAG_TWO_FACTOR_AUTH: i32 = 4;
pub const USER_FLAG_EMAIL_VERIFIED: i32 = 8;

/// User details.
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub secret: Option<String>,
    pub flags: i32,
}

impl User {
    pub fn has_flag(&self, flag: i32) -> bool {
        self.flags & flag == flag
    }

    pub fn set_flag(&mut self, flag: i32) {
        self.flags |= flag;
    }

    pub fn unset_flag(&mut self, flag: i32) {
        self.flags &= !flag;
    }

    pub fn get_gravatar_hash(&self) -> String {
        format!(
            "{:x}",
            md5::compute(self.email.trim().to_lowercase().as_bytes())
        )
    }

    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "id": self.id,
            "username": self.username,
            "email": self.email,
            "flags": self.flags,
            "admin": self.has_flag(USER_FLAG_ADMIN),
            "suspended": self.has_flag(USER_FLAG_SUSPENDED),
            "two_factor_auth": self.has_flag(USER_FLAG_TWO_FACTOR_AUTH),
            "email_verified": self.has_flag(USER_FLAG_EMAIL_VERIFIED),
        })
    }
}

/// New user details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
    pub secret: Option<String>,
    pub flags: i32,
}

impl NewUser {
    /// Constructs new user details from name.
    #[cfg(test)] // only needed in tests
    pub fn new(
        username: impl Into<String>,
        email: impl Into<String>,
        password: impl Into<String>,
        secret: Option<String>,
        flags: i32,
    ) -> Self {
        Self {
            username: username.into(),
            email: email.into(),
            password: password.into(),
            secret,
            flags,
        }
    }
}
