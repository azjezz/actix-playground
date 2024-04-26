use askama::Template;

#[derive(Template)]
#[template(path = "error/404.html")]
pub struct NotFoundErrorTemplate;
