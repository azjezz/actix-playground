use diesel::{deserialize::Queryable, prelude::Insertable};
use md5;
use serde::{Deserialize, Serialize, Serializer};

use crate::database::schema::users;

pub const USER_FLAG_ADMIN: i32 = 1;
pub const USER_FLAG_SUSPENDED: i32 = 2;
pub const USER_FLAG_TWO_FACTOR_AUTH: i32 = 4;
pub const USER_FLAG_EMAIL_VERIFIED: i32 = 8;

/// User details.
#[derive(Debug, Clone, Deserialize, Queryable, Insertable)]
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
}

impl Serialize for User {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("User", 6)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("username", &self.username)?;
        state.serialize_field("email", &self.email)?;
        state.serialize_field("password", &self.password)?;
        state.serialize_field("secret", &self.secret)?;
        state.serialize_field("flags", &self.flags)?;
        state.serialize_field("admin", &self.has_flag(USER_FLAG_ADMIN))?;
        state.serialize_field("suspended", &self.has_flag(USER_FLAG_SUSPENDED))?;
        state.serialize_field("two_factor_auth", &self.has_flag(USER_FLAG_TWO_FACTOR_AUTH))?;
        state.serialize_field("email_verified", &self.has_flag(USER_FLAG_EMAIL_VERIFIED))?;
        state.serialize_field("avatar_hash", &self.get_gravatar_hash())?;
        state.end()
    }
}
