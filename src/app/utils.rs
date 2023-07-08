use humansize::{format_size, DECIMAL};
use mime_guess::{mime, MimeGuess};
use rocket::{response::content::RawHtml, State};
use std::path::{Path, PathBuf};
use tera::{Context, Tera};
use tokio::fs::read_dir;

pub async fn find_file_with_key(dir_path: PathBuf, key: &String) -> Option<PathBuf> {
  let key = format!(
    "-{}",
    key
      .trim_end_matches('.')
      .rsplit_once('.')
      .map(|(name, _)| name)
      .unwrap_or(key)
  );
  let prefixes = vec!["Images", "Videos", "Audio", "Text", "Files"];

  for prefix in prefixes {
    let sub_dir_path = dir_path.join(prefix);
    let mut files = match read_dir(&sub_dir_path).await {
      Ok(files) => files,
      Err(_) => continue,
    };

    while let Some(entry) = files.next_entry().await.unwrap_or(None) {
      if let Some(file_name) = entry.file_name().to_str() {
        let basename = Path::new(file_name)
          .file_stem()
          .and_then(|stem| stem.to_str())
          .unwrap_or("");
        if basename.ends_with(&key) {
          let file_path = sub_dir_path.join(file_name);
          return Some(file_path);
        }
      }
    }
  }

  None
}

pub enum FileType {
  Image,
  Video,
  Audio,
  Text,
  File,
}

pub struct FileMetadata {
  pub file_type: FileType,
  pub filesize: String,
  pub filename: String,
}

pub fn get_file_type(path: &Path) -> FileType {
  let mime_type = MimeGuess::from_path(path).first_or_octet_stream();
  match mime_type.type_() {
    mime::IMAGE => FileType::Image,
    mime::VIDEO => FileType::Video,
    mime::AUDIO => FileType::Audio,
    mime::TEXT => FileType::Text,
    _ => {
      if is_text_file(path) {
        FileType::Text
      } else {
        FileType::File
      }
    }
  }
}

pub fn get_file_metadata(path: &Path) -> FileMetadata {
  let filename = match path.file_name() {
    Some(name) => {
      let name = name.to_string_lossy();
      match name.rfind('-') {
        Some(index) => {
          let filename = &name[..index];
          let extension = &name[index..];
          let extension_start = extension.rfind('.').unwrap_or(extension.len());
          format!("{}{}", filename, &extension[extension_start..])
        }
        None => name.clone().to_string(),
      }
    }
    None => String::new(),
  };

  let filesize = format_size(
    match path.metadata() {
      Ok(metadata) => metadata.len(),
      Err(_) => 0,
    },
    DECIMAL,
  );

  let file_type = get_file_type(path);

  FileMetadata {
    file_type,
    filesize,
    filename,
  }
}

pub fn render_not_found(tera: &State<Tera>) -> RawHtml<String> {
  let mut context = Context::new();
  context.insert("title", "404 Not Found");
  context.insert("header", "404");
  context.insert("subheader", "PAGE NOT FOUND");
  context.insert("text", "The page you were looking for does not exist");
  RawHtml(tera.render("Error", &context).unwrap())
}

// ChatGPT generated this list for me :3
const TEXT_EXTENSIONS: [&str; 77] = [
  "c", "cpp", "h", "hpp", "cc", "cxx", "hxx",  // C/C++
  "cs",   // C#
  "java", // Java
  "py",   // Python
  "php",  // PHP
  "rb",   // Ruby
  "js", "jsx", "ts", "tsx", // JavaScript/TypeScript
  "html", "htm", "css",  // HTML/CSS
  "xml",  // XML
  "json", // JSON
  "yaml", "yml",   // YAML
  "sql",   // SQL
  "swift", // Swift
  "go",    // Go
  "rust", "rs",     // Rust
  "scala",  // Scala
  "kotlin", // Kotlin
  "perl",   // Perl
  "lua",    // Lua
  "sh", "bash", // Shell scripts
  "r",    // R
  "mat",  // MATLAB
  "asm",  // Assembly
  "pl",   // Prolog
  "vb",   // Visual Basic
  "fs", "fsx",  // F#
  "dart", // Dart
  "jl",   // Julia
  "ml", "mli", // OCaml
  "clj", "cljs",   // Clojure
  "coffee", // CoffeeScript
  "groovy", // Groovy
  "pde",    // Processing
  "hs",     // Haskell
  "erl", "hrl", // Erlang
  "ex", "exs", // Elixir
  "lisp", "lsp",  // Lisp
  "cr",   // Crystal
  "v",    // Verilog
  "dart", // Dart
  "sc",   // Supercollider
  "awk",  // AWK
  "ini", "cfg", "conf", // Configuration files
  "bat", "cmd", // Batch files
  "ps1", "psm1",   // PowerShell
  "groovy", // Groovy
  "ejs",    // Embedded JavaScript
  "md", "markdown", // Markdown
  "tex", "sty", // LaTeX
  "svg", // SVG
  "txt", // Plain text
];

fn is_text_file(path: &Path) -> bool {
  if let Some(extension) = path.extension() {
    if let Some(extension_str) = extension.to_str() {
      return TEXT_EXTENSIONS.contains(&extension_str.to_lowercase().as_str());
    }
  }
  false
}
