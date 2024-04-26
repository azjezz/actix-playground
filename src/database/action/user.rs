use diesel::prelude::*;
use uuid::Uuid;

use crate::database::model::user::User;
use crate::database::Result;

pub fn insert_new_user(
    conn: &mut SqliteConnection,
    new_username: &str,
    new_email: &str,
    new_password: &str,
) -> Result<User> {
    use crate::database::schema::users::dsl::*;

    let new_user = User {
        id: Uuid::new_v4().to_string(),
        username: new_username.to_string(),
        email: new_email.to_string(),
        password: new_password.to_string(),
        secret: None,
        flags: 0,
    };

    diesel::insert_into(users).values(&new_user).execute(conn)?;

    Ok(new_user)
}

pub fn get_user_by_id(conn: &mut SqliteConnection, user_id: &str) -> Result<Option<User>> {
    use crate::database::schema::users::dsl::{id, users};

    let user = users
        .filter(id.eq(user_id))
        .first::<User>(conn)
        .optional()?;

    Ok(user)
}

pub fn get_user_by_email(conn: &mut SqliteConnection, email: &str) -> Result<Option<User>> {
    use crate::database::schema::users::dsl::{email as email_column, users};

    let user = users
        .filter(email_column.eq(email))
        .first::<User>(conn)
        .optional()?;

    Ok(user)
}
