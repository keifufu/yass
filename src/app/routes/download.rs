use rocket::{
  fs::NamedFile,
  response::{content::RawHtml, status::NotFound},
  State,
};
use tera::Tera;

use crate::{
  app::{
    responders::attachment::Attachment,
    utils::{find_file_with_key, get_file_metadata, render_not_found},
  },
  AppConfig,
};

#[get("/download/<key>")]
pub async fn download_route(
  key: String,
  config: &State<AppConfig>,
  tera: &State<Tera>,
) -> Result<Attachment<NamedFile>, NotFound<RawHtml<String>>> {
  let file_path = find_file_with_key(config.data_path.clone(), &key).await;

  if let Some(path) = file_path {
    let metadata = get_file_metadata(&path);
    Ok(Attachment {
      responder: NamedFile::open(path).await.unwrap(),
      filename: metadata.filename,
    })
  } else {
    Err(NotFound(render_not_found(tera)))
  }
}
