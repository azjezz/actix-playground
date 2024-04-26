use crate::database::model::user::User;
use actix_utils::future::{ready, Ready};
use actix_web::{dev::Payload, Error, FromRequest, HttpMessage, HttpRequest};

#[derive(Debug, Clone)]
pub enum SecurityToken {
    Anonymous,
    Authenticated { user: User },
}

impl SecurityToken {
    pub fn is_authenticated(&self) -> bool {
        matches!(self, SecurityToken::Authenticated { .. })
    }

    pub fn user(&self) -> Option<User> {
        match self {
            SecurityToken::Authenticated { user } => Some(user.clone()),
            _ => None,
        }
    }
}

impl FromRequest for SecurityToken {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        if let Some(user) = req.extensions().get::<User>() {
            ready(Ok(SecurityToken::Authenticated { user: user.clone() }))
        } else {
            ready(Ok(SecurityToken::Anonymous))
        }
    }
}
