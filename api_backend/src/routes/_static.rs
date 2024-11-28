use rocket::fs::NamedFile;
use std::path::Path;

#[get("/favicon.ico")]
pub async fn favicon() -> Option<NamedFile> {
  NamedFile::open(Path::new("./static/favicon.ico")).await.ok()
}
