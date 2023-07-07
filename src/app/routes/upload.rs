use std::path::Path;
use tokio::fs::create_dir_all;

use rand::distributions::{Alphanumeric, DistString};
use rocket::{fs::TempFile, State};

use crate::{
  app::{
    utils::{get_file_type, FileType},
    validators::apikey::ApiKey,
  },
  AppConfig,
};

#[put("/upload?<filename>", data = "<file>")]
pub async fn upload_route(
  _api_key: ApiKey<'_>,
  filename: String,
  mut file: TempFile<'_>,
  config: &State<AppConfig>,
) -> std::io::Result<String> {
  let path: &Path = Path::new(&filename);
  let basename = path
    .file_stem()
    .and_then(|filename| filename.to_str())
    .unwrap_or("");
  let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("");
  let id = Alphanumeric.sample_string(&mut rand::thread_rng(), 7);
  let new_file_name = format!("{}-{}.{}", basename, id, extension);

  let file_path = match get_file_type(path) {
    FileType::Image => config.data_path.join("Images").join(new_file_name),
    FileType::Video => config.data_path.join("Videos").join(new_file_name),
    FileType::Audio => config.data_path.join("Audio").join(new_file_name),
    FileType::Text => config.data_path.join("Text").join(new_file_name),
    FileType::File => config.data_path.join("Files").join(new_file_name),
  };

  if let Some(parent) = file_path.parent() {
    if let Err(err) = create_dir_all(parent).await {
      eprintln!("Failed to create parent folders: {}", err);
    }
  }

  file.persist_to(file_path).await?;

  Ok(format!(
    "{}/{}.{}",
    config.url.strip_suffix('/').unwrap_or(&config.url),
    id,
    extension
  ))
}
