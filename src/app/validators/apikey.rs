use rocket::{
  http::Status,
  request::{FromRequest, Outcome, Request},
  State,
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
    let config = match request.guard::<&State<AppConfig>>().await.succeeded() {
      Some(config) => config,
      None => return Outcome::Failure((Status::InternalServerError, ApiKeyError::Missing)),
    };
    match request.headers().get_one("Authorization") {
      None => Outcome::Failure((Status::BadRequest, ApiKeyError::Missing)),
      Some(api_key) if api_key == config.api_key => Outcome::Success(ApiKey(api_key)),
      Some(_) => Outcome::Failure((Status::BadRequest, ApiKeyError::Invalid)),
    }
  }
}
