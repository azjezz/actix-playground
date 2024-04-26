use crate::database::model::user::User;
use actix_utils::future::{ready, Ready};
use actix_web::{dev::Payload, Error, FromRequest, HttpMessage, HttpRequest};

#[derive(Debug, Clone)]
pub enum SecurityContext {
    Anonymous,
    Authenticated { user: User },
}

impl FromRequest for SecurityContext {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        if let Some(user) = req.extensions().get::<User>() {
            ready(Ok(SecurityContext::Authenticated { user: user.clone() }))
        } else {
            ready(Ok(SecurityContext::Anonymous))
        }
    }
}
