use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

#[derive(Debug)]
pub enum AuthError {
    Forbidden,
    TokenExpired,
    NoTokenFound,
}

#[derive(Debug)]
pub struct AuthGuard {
    pub token: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthGuard {
    type Error = AuthError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match req
            .cookies()
            .get_private("auth_token")
            .and_then(|c| c.value().parse().ok())
        {
            Some(token) => Outcome::Success(AuthGuard { token }),
            None => Outcome::Failure((Status::Unauthorized, AuthError::Forbidden)),
        }
    }
}
