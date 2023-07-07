use rocket::{
  response::{
    content::{RawCss, RawHtml, RawJavaScript},
    status::NotFound,
  },
  State,
};
use tera::Tera;

use crate::app::utils::render_not_found;

#[get("/assets/css/<name>")]
pub async fn assets_css_route(
  name: String,
  tera: &State<Tera>,
) -> Result<RawCss<String>, NotFound<RawHtml<String>>> {
  if name == "base.css" {
    Ok(RawCss(include_str!("../assets/base.css").to_string()))
  } else {
    Err(NotFound(render_not_found(tera)))
  }
}

#[get("/assets/js/<name>")]
pub async fn assets_js_route(
  name: String,
  tera: &State<Tera>,
) -> Result<RawJavaScript<String>, NotFound<RawHtml<String>>> {
  if name == "theme.js" {
    Ok(RawJavaScript(
      include_str!("../assets/theme.js").to_string(),
    ))
  } else {
    Err(NotFound(render_not_found(tera)))
  }
}
