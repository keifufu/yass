use rocket::{
  http::Status,
  request::{FromRequest, Outcome, Request},
};

use crate::AppConfig;

#[derive(Debug)]
pub struct ApiKey<'r>(&'r str);
#[derive(Debug)]
pub enum ApiKeyError {
  Missing,
  Invalid,
}
#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey<'r> {
  type Error = ApiKeyError;
  async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
    let config = AppConfig::get();
    match request.headers().get_one("Authorization") {
      None => Outcome::Failure((Status::Unauthorized, ApiKeyError::Missing)),
      Some(api_key) if api_key == config.api_key => Outcome::Success(ApiKey(api_key)),
      Some(_) => Outcome::Failure((Status::Unauthorized, ApiKeyError::Invalid)),
    }
  }
}
