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
use std::{
  env::current_exe,
  fs::{create_dir, File},
  io::Write,
};
use std::{fs::OpenOptions, path::PathBuf};
use tera::Tera;

#[macro_use]
extern crate rocket;
mod app;

const DEFAULT_CONFIG: &str = include_str!("data/config.toml");

#[derive(Debug)]
pub struct AppConfig {
  pub data_path: PathBuf,
  pub api_key: String,
  pub url: String,
  pub port: usize,
  pub key_length: usize,
  pub rpm_view: u32,
  pub rpm_upload: u32,
  pub block_wei_browsers: bool,
}
static APP_CONFIG: Lazy<AppConfig> = Lazy::new(AppConfig::read);
impl AppConfig {
  fn read() -> Self {
    let exe_path = current_exe().unwrap();
    let current_dir = exe_path.parent().unwrap();
    let config_dir = current_dir.join("config");
    let config_path = config_dir.join("config.toml");

    if !config_dir.exists() {
      create_dir(&config_dir).unwrap();
    }

    if !config_path.exists() {
      let mut file = File::create(&config_path).unwrap();
      file.write_all(DEFAULT_CONFIG.as_bytes()).unwrap();
    }

    let cfg = Config::builder()
      .add_source(config::File::from(config_path.clone()))
      .build()
      .unwrap();

    if cfg.get_bool("block_wei_browsers").is_err() {
      let mut file = OpenOptions::new()
        .append(true)
        .open(&config_path)
        .expect("Failed to open config file");

      // Make sure a newline exists
      #[allow(clippy::writeln_empty_string)]
      if let Err(err) = writeln!(file, "") {
        panic!("Failed to write to config file: {}", err);
      }

      if let Err(err) = writeln!(
        file,
        "# Block browsers supporting the Web Environment Integrity API"
      ) {
        panic!("Failed to write to config file: {}", err);
      }

      if let Err(err) = writeln!(file, "block_wei_browsers = true") {
        panic!("Failed to write to config file: {}", err);
      }
    }

    let config = Config::builder()
      .add_source(config::File::from(config_path.clone()))
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
      block_wei_browsers: config.get_bool("block_wei_browsers").unwrap(),
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
