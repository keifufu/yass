use app::routes::{
  assets::{assets_css_route, assets_js_route},
  catchers::{internal_error_catcher, not_found_catcher, rate_limit_catcher},
  download::download_route,
  upload::upload_route,
  view::view_route,
};
use config::Config;
use once_cell::sync::Lazy;
use rocket::data::{ByteUnit, Limits};
use std::path::PathBuf;
use tera::Tera;

#[macro_use]
extern crate rocket;
mod app;

#[derive(Debug)]
pub struct AppConfig {
  pub data_path: PathBuf,
  pub api_key: String,
  pub url: String,
  pub port: usize,
  pub key_length: usize,
  pub rpm_view: u32,
  pub rpm_upload: u32,
}
static APP_CONFIG: Lazy<AppConfig> = Lazy::new(AppConfig::read);
impl AppConfig {
  fn read() -> Self {
    let config = Config::builder()
      .add_source(config::File::with_name("config.toml"))
      .build()
      .unwrap();
    Self {
      data_path: PathBuf::from(config.get_string("data-path").unwrap()),
      api_key: config.get_string("api-key").unwrap(),
      url: config.get_string("url").unwrap(),
      port: config.get_int("port").unwrap() as usize,
      key_length: config.get_int("key-length").unwrap() as usize,
      rpm_view: config.get_int("rpm-view").unwrap() as u32,
      rpm_upload: config.get_int("rpm-upload").unwrap() as u32,
    }
  }

  pub fn get() -> &'static Self {
    &APP_CONFIG
  }
}

#[launch]
fn rocket() -> _ {
  let config = AppConfig::get();

  let mut tera = Tera::default();
  tera
    .add_raw_templates(vec![
      ("Error", include_str!("./app/templates/Error.html.tera")),
      ("Image", include_str!("./app/templates/Image.html.tera")),
      ("Video", include_str!("./app/templates/Video.html.tera")),
      ("Audio", include_str!("./app/templates/Audio.html.tera")),
      ("Text", include_str!("./app/templates/Text.html.tera")),
      ("File", include_str!("./app/templates/File.html.tera")),
    ])
    .unwrap();

  let figment = rocket::Config::figment()
    .merge(("port", config.port))
    .merge(("address", "0.0.0.0"))
    .merge((
      "limits",
      Limits::new().limit("file", ByteUnit::Gigabyte(1000)),
    ));

  rocket::custom(figment)
    .mount(
      "/",
      routes![
        upload_route,
        view_route,
        download_route,
        assets_css_route,
        assets_js_route
      ],
    )
    .register(
      "/",
      catchers![
        internal_error_catcher,
        not_found_catcher,
        rate_limit_catcher
      ],
    )
    .manage(tera)
    .attach(rocket_governor::LimitHeaderGen::default())
}
