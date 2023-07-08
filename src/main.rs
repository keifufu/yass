use app::routes::{
  assets::{assets_css_route, assets_js_route},
  catchers::{internal_error_catcher, not_found_catcher},
  download::download_route,
  upload::upload_route,
  view::view_route,
};
use config::Config;
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
}

#[launch]
fn rocket() -> _ {
  let config = Config::builder()
    .add_source(config::File::with_name("config.toml"))
    .build()
    .unwrap();

  let config = AppConfig {
    data_path: PathBuf::from(config.get_string("data-path").unwrap()),
    api_key: config.get_string("api-key").unwrap(),
    url: config.get_string("url").unwrap(),
    port: config.get_int("port").unwrap() as usize,
    key_length: config.get_int("key-length").unwrap() as usize,
  };

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
    .register("/", catchers![internal_error_catcher, not_found_catcher])
    .manage(config)
    .manage(tera)
}
