use std::collections::HashMap;
use std::path::Path;

use rocket::form::Form;
use rocket::fs::relative;
use rocket::fs::NamedFile;
use rocket::get;
use rocket::post;
use rocket::response::content;

#[get("/")]
pub async fn index() -> Option<NamedFile> {
    let path = Path::new(relative!("public/csvmfr/index.html"));
    NamedFile::open(path).await.ok()
}

#[post("/add_stage", data = "<form>")]
pub async fn add_stage(form: Form<HashMap<String, String>>) -> Option<content::RawHtml<String>> {
    let form: HashMap<String, String> = form.into_inner();

    let stage_type = form.get("stage_type")?;

    Some(content::RawHtml(get_text_box(stage_type)))
}

fn get_text_box(id: impl Into<String>) -> String {
    format!("<textarea id=\"{}\"></textarea>", id.into())
}
