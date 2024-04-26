use askama::Template;

pub mod error;
pub mod user;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate;
