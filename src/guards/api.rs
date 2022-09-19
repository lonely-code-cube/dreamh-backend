// use std::env;

// use rocket::http::Status;
// use rocket::request::{FromRequest, Outcome, Request};

// lazy_static! {
//     static ref FRONTEND_ID: String =
//         env::var("FRONTEND_ID").expect("FRONTEND_ID environment variable missing");
// }

// pub struct Client {
//     pub is_frontend: bool,
//     pub api_key: Option<String>,
//     pub max_uses: usize,
//     pub uses: usize,
// }

// pub struct Frontend;

// #[derive(Debug)]
// pub enum ClientKeyError {
//     Missing,
//     Expired,
//     Invalid,
// }

// #[rocket::async_trait]
// impl<'r> FromRequest<'r> for Frontend {
//     type Error = std::convert::Infallible;

//     async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
//         fn is_valid(id: &str) -> bool {
//             constant_time_eq::constant_time_eq(id.as_bytes(), &FRONTEND_ID.as_bytes())
//         }

//         match req.headers().get_one("X-Frontend-Pass") {
//             None => Outcome::Forward(()),
//             Some(pass) if is_valid(pass) => Outcome::Success(Frontend {}),
//             Some(_) => Outcome::Forward(()),
//         }
//     }
// }

// #[rocket::async_trait]
// impl<'r> FromRequest<'r> for ApiKey {
//     type Error = ApiKeyError;

//     async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
//         fn is_valid(key: &str) -> bool {
//             true
//         }

//         match req.headers().get_one("X-Api-Key") {
//             None => Outcome::Failure((Status::Unauthorized, ApiKeyError::Missing)),
//             Some(key) if is_valid(key) => Outcome::Success(ApiKey {
//                 api_key: key.to_owned(),
//             }),
//             Some(_) => Outcome::Failure((Status::Unauthorized, ApiKeyError::Invalid)),
//         }
//     }
// }
