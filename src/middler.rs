use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::response;
use rocket::response::Redirect;
use rocket::response::Responder;
use rocket::Request;
use rocket::Response;
use std::io::Cursor;

#[derive(Debug)]
pub struct ApiKey<'a>(pub &'a str);

#[derive(Debug)]
pub enum ApiKeyError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey<'r> {
    type Error = ApiKeyError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        fn is_valid(key: &str) -> bool {
            key == "123456"
        }

        match req.headers().get_one("x-api-key") {
            None => Outcome::Error((Status::BadRequest, ApiKeyError::Missing)),
            Some(key) if is_valid(key) => Outcome::Success(ApiKey(key)),
            Some(_) => Outcome::Error((Status::BadRequest, ApiKeyError::Invalid)),
        }
    }
}

pub enum MyResult {
    Ok(String),
    Err(String),
    Redirect(Box<Redirect>),
}

#[rocket::async_trait]
impl<'r> Responder<'r, 'static> for MyResult {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        match self {
            MyResult::Ok(s) => Response::build()
                .status(Status::Ok)
                .sized_body(s.len(), Cursor::new(s))
                .ok(),
            MyResult::Err(s) => Response::build()
                .status(Status::BadRequest)
                .sized_body(s.len(), Cursor::new(s))
                .ok(),
            MyResult::Redirect(r) => r.respond_to(req),
        }
    }
}
