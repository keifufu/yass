use rocket::{
  response::{
    content::{RawCss, RawHtml, RawJavaScript},
    status::NotFound,
  },
  State,
};
use rocket_governor::RocketGovernor;
use tera::Tera;

use crate::{
  app::{utils::render_not_found, validators::ratelimit::RateLimitGuard},
  AppConfig,
};

#[get("/assets/css/<name>")]
pub async fn assets_css_route(
  _rl: RocketGovernor<'_, RateLimitGuard>,
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
  _rl: RocketGovernor<'_, RateLimitGuard>,
  name: String,
  tera: &State<Tera>,
) -> Result<RawJavaScript<String>, NotFound<RawHtml<String>>> {
  if name == "theme.js" {
    Ok(RawJavaScript(
      include_str!("../assets/theme.js").to_string(),
    ))
  } else if name == "wei.js" {
    if AppConfig::get().block_wei_browsers {
      Ok(RawJavaScript(include_str!("../assets/wei.js").to_string()))
    } else {
      Err(NotFound(render_not_found(tera)))
    }
  } else {
    Err(NotFound(render_not_found(tera)))
  }
}
