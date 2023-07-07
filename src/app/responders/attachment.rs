use rocket::http::Header;
use rocket::{
  response::{Responder, Result},
  Request, Response,
};
use std::fmt::Debug;

#[derive(Debug)]
pub struct Attachment<R> {
  pub responder: R,
  pub filename: String,
}
impl<R> Attachment<R> {}
impl<'r, 'req, R> Responder<'r, 'req> for Attachment<R>
where
  R: Debug + Responder<'r, 'req>,
  'req: 'r,
{
  fn respond_to(self, req: &'r Request<'_>) -> Result<'req> {
    Response::build()
      .merge(self.responder.respond_to(req)?)
      .header(Header::new(
        "Content-Disposition",
        format!("attachment; filename=\"{}\"", self.filename),
      ))
      .ok()
  }
}
