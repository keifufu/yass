use rocket::{
  fs::NamedFile,
  response::{content::RawHtml, status::NotFound, Redirect},
  Either, State,
};
use rocket_governor::RocketGovernor;
use tera::{Context, Tera};
use tokio::fs::read_to_string;

use crate::{
  app::{
    utils::{find_file_with_key, get_file_metadata, render_not_found, FileType},
    validators::{botrequest::IsBotRequest, ratelimit::RateLimitGuard},
  },
  AppConfig,
};

#[get("/<key>?<raw>")]
pub async fn view_route(
  _rl: RocketGovernor<'_, RateLimitGuard>,
  key: String,
  is_bot: IsBotRequest,
  tera: &State<Tera>,
  raw: bool,
) -> Result<Either<RawHtml<String>, NamedFile>, Either<Redirect, NotFound<RawHtml<String>>>> {
  if raw {
    let config = AppConfig::get();
    let file_path = find_file_with_key(config.data_path.clone(), &key).await;
    if let Some(path) = file_path {
      return Ok(Either::Right(NamedFile::open(path).await.unwrap()));
    }
  }

  if is_bot.0 {
    return Err(Either::Left(Redirect::to(format!("/download/{}", key))));
  }

  let config = AppConfig::get();
  let file_path = find_file_with_key(config.data_path.clone(), &key).await;
  if let Some(path) = file_path {
    let metadata = get_file_metadata(&path);
    let mut context = Context::new();
    let template = match metadata.file_type {
      FileType::Image => "Image",
      FileType::Video => "Video",
      FileType::Audio => "Audio",
      FileType::Text => "Text",
      FileType::File => "File",
    };
    if template == "Text" {
      let contents = read_to_string(path).await.unwrap();
      let escaped = html_escape::encode_text(&contents);
      context.insert("contents", &escaped);
    }
    context.insert("title", &metadata.filename);
    context.insert("filename", &metadata.filename);
    context.insert("filesize", &metadata.filesize);
    context.insert("key", &key);
    Ok(Either::Left(RawHtml(
      tera.render(template, &context).unwrap(),
    )))
  } else {
    Err(Either::Right(NotFound(render_not_found(tera))))
  }
}
