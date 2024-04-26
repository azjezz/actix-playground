use crate::database::{self, model::user::User};
use actix_session::SessionExt;
use actix_utils::future::{ready, Ready};
use actix_web::{
    dev::{forward_ready, Payload, Service, ServiceRequest, ServiceResponse, Transform},
    web, Error, FromRequest, HttpMessage, HttpRequest,
};
use std::{future::Future, pin::Pin};

// Middleware Factory
pub struct UserDataMiddleware;

// Middleware Service
pub struct UserDataService<S> {
    service: S,
}

impl<S, B> Transform<S, ServiceRequest> for UserDataMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = UserDataService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(UserDataService { service }))
    }
}

impl<S, B> Service<ServiceRequest> for UserDataService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let session = req.get_session();

        if let Some(user_id) = session.get::<String>("user_id").unwrap_or(None) {
            let pool = req
                .app_data::<web::Data<database::Pool>>()
                .expect("couldn't get db pool from request");
            let mut connection = pool.get().expect("couldn't get db connection from pool");
            if let Ok(Some(user)) =
                database::action::user::get_user_by_id(&mut connection, &user_id)
            {
                req.extensions_mut().insert::<User>(user);
            } else {
                session.remove("user_id");
            }
        }

        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}

impl FromRequest for User {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        if let Some(user) = req.extensions().get::<User>() {
            ready(Ok(user.clone()))
        } else {
            ready(Err(actix_web::error::ErrorUnauthorized("Unauthorized")))
        }
    }
}
