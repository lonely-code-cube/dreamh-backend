use std::env;

use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

pub struct Admin;

#[derive(Debug)]
pub enum LoginError {
    Missing,
    Invalid,
}

lazy_static! {
    static ref USERNAME: String =
        env::var("USERNAME").expect("USERNAME environment variable not set");
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Admin {
    type Error = LoginError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        fn is_valid(user: &str) -> bool {
            constant_time_eq::constant_time_eq(user.as_bytes(), "admin".as_bytes())
        }

        match req.cookies().get_private("auth") {
            Some(cookie) if is_valid(cookie.value()) => Outcome::Success(Admin {}),
            Some(_) => Outcome::Failure((Status::Unauthorized, LoginError::Invalid)),
            None => Outcome::Failure((Status::Unauthorized, LoginError::Missing)),
        }
    }
}
