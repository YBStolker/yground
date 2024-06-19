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

#[get("/get_pipeline_stage")]
pub async fn get_pipeline_stage() -> Option<RawHtml<String>> {
    let result = TEMPLATES
        .render("csv_mfr/pipeline_stage.html", &Context::default())
        .ok()
        .map(|value| RawHtml(value));

    result
}
