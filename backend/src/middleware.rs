use actix_web::{FromRequest, HttpRequest};
use actix_web::dev::Payload;
use actix_web::error::ErrorUnauthorized;
use futures_util::future::{ready, Ready};
use uuid::Uuid;
use crate::auth;

pub struct AuthenticatedUser {
    pub user_id: Uuid,
}

impl FromRequest for AuthenticatedUser {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        match auth::extract_user_id_from_request(req) {
            Ok(user_id) => ready(Ok(AuthenticatedUser { user_id })),
            Err(e) => ready(Err(e)),
        }
    }
}

