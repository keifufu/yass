use rocket_governor::RocketGovernor;
use std::path::Path;
use tokio::fs::create_dir_all;

use async_recursion::async_recursion;
use rand::distributions::{Alphanumeric, DistString};
use rocket::{fs::TempFile, response::status::Conflict};

use crate::{
  app::{
    utils::{find_file_with_key, get_file_type, FileType},
    validators::{apikey::ApiKey, ratelimit::RateLimitGuard},
  },
  AppConfig,
};

#[put("/upload?<filename>", data = "<file>")]
pub async fn upload_route(
  _rl: RocketGovernor<'_, RateLimitGuard>,
  _api_key: ApiKey<'_>,
  filename: String,
  mut file: TempFile<'_>,
) -> Result<String, Conflict<String>> {
  let config = AppConfig::get();
  let key = match get_key(config, 0).await {
    Some(key) => key,
    None => return Err(Conflict(None)),
  };
  let path: &Path = Path::new(&filename);
  let basename = path
    .file_stem()
    .and_then(|filename| filename.to_str())
    .unwrap_or("");
  let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("");
  let new_file_name = format!("{}-{}.{}", basename, key, extension);

  let file_path = match get_file_type(path) {
    FileType::Image => config.data_path.join("Images").join(new_file_name),
    FileType::Video => config.data_path.join("Videos").join(new_file_name),
    FileType::Audio => config.data_path.join("Audio").join(new_file_name),
    FileType::Text => config.data_path.join("Text").join(new_file_name),
    FileType::File => config.data_path.join("Files").join(new_file_name),
  };

  if let Some(parent) = file_path.parent() {
    if create_dir_all(parent).await.is_err() {
      return Err(Conflict(None));
    }
  }

  // Note: persist_to will not work since it can't handle moving it to a different disk.
  // OsCode 18 on linux, OsCode 17 on windows. move_copy_to fixes this.
  // Another way would be to set the temp_dir to the same drive, but then there is no guarantee
  // that the temp directory will be cleaned up. Rocket does seem to clean it if an upload gets
  // cancelled but what if yass crashes? yeah. move_copy_to is fine.
  file.move_copy_to(file_path).await.unwrap();

  Ok(format!(
    "{}/{}.{}",
    config.url.strip_suffix('/').unwrap_or(&config.url),
    key,
    extension
  ))
}

#[async_recursion]
async fn get_key(config: &AppConfig, depth: usize) -> Option<String> {
  if depth > 10 {
    return None;
  }
  let key = Alphanumeric.sample_string(&mut rand::thread_rng(), config.key_length);
  if find_file_with_key(config.data_path.clone(), &key)
    .await
    .is_some()
  {
    return get_key(config, depth + 1).await;
  }
  Some(key)
}
