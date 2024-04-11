use std::path::Path;

use rocket::fs::relative;
use rocket::fs::NamedFile;
use rocket::get;
use rocket::response::content::RawHtml;
use tera::Context;

use crate::TEMPLATES;

#[get("/")]
pub async fn index() -> Option<NamedFile> {
    let path = Path::new(relative!("public/csv_mfr/index.html"));
    NamedFile::open(path).await.ok()
}

fn capitalize(s: impl Into<String>) -> String {
    let s: String = s.into();
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
    }
}

#[get("/get_pipeline_stage/<stage_type>")]
pub async fn get_pipeline_stage(stage_type: &str) -> Option<RawHtml<String>> {
    let stage_title = capitalize(stage_type);

    let mut context = Context::new();
    context.insert("stage_title", stage_title.as_str());
    context.insert("stage_type", stage_type);

    let result = TEMPLATES
        .render("csv_mfr/pipeline_stage.html", &context)
        .ok()
        .map(|value| RawHtml(value));

    result
}
