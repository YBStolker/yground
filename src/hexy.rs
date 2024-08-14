use std::path::Path;

use rocket::fs::relative;
use rocket::fs::NamedFile;
use rocket::get;

// Hexy file
#[get("/")]
pub async fn index() -> Option<NamedFile> {
    let path = Path::new(relative!("public/hexy/index.html"));
    NamedFile::open(path).await.ok()
}
