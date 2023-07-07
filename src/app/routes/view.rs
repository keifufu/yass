use rocket::{
  response::{content::RawHtml, status::NotFound, Redirect},
  Either, State,
};
use tera::{Context, Tera};
use tokio::fs::read_to_string;

use crate::{
  app::{
    utils::{find_file_with_id, get_file_metadata, render_not_found, FileType},
    validators::botrequest::IsBotRequest,
  },
  AppConfig,
};

#[get("/<id>")]
pub async fn view_route(
  id: String,
  is_bot: IsBotRequest,
  config: &State<AppConfig>,
  tera: &State<Tera>,
) -> Result<RawHtml<String>, Either<Redirect, NotFound<RawHtml<String>>>> {
  if is_bot.0 {
    return Err(Either::Left(Redirect::to(format!("/download/{}", id))));
  }

  let file_path = find_file_with_id(config.data_path.clone(), &id).await;
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
    context.insert("id", &id);
    Ok(RawHtml(tera.render(template, &context).unwrap()))
  } else {
    Err(Either::Right(NotFound(render_not_found(tera))))
  }
}
