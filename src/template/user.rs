use crate::database::model::user;
use askama::Template;
use tarjama::{locale::Locale, Translator};

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate;

#[derive(Template)]
#[template(path = "register.html")]
pub struct RegisterTemplate;

#[derive(Template)]
#[template(path = "profile.html")]
pub struct ProfileTemplate {
    pub translator: Translator,
    pub locale: Locale,
    pub user: user::User,
}
