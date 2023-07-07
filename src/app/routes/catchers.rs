use rocket::{response::content::RawHtml, Request, State};
use tera::{Context, Tera};

use crate::app::utils::render_not_found;

#[catch(500)]
pub async fn internal_error_catcher(req: &Request<'_>) -> RawHtml<String> {
  let tera = req.guard::<&State<Tera>>().await.unwrap();
  let mut context = Context::new();
  context.insert("title", "500 Internal Server Error");
  context.insert("header", "500");
  context.insert("subheader", "An error occurred");
  context.insert("text", "Check your request or try again later");
  RawHtml(tera.render("Error", &context).unwrap())
}

#[catch(404)]
pub async fn not_found_catcher(req: &Request<'_>) -> RawHtml<String> {
  let tera = req.guard::<&State<Tera>>().await.unwrap();
  render_not_found(tera)
}
