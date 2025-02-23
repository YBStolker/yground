use std::path::Path;

use rocket::fs::relative;
use rocket::fs::NamedFile;
use rocket::get;
use rocket::response::content::RawHtml;
use rocket::routes;
use rocket::Route;
use tera::Context;

use crate::TEMPLATES;

pub fn get_routes() -> Vec<Route> {
    routes![index, get_pipeline_stage]
}

#[get("/")]
pub async fn index() -> Option<NamedFile> {
    let path = Path::new(relative!("public/csv_mfr/csv_mfr.html"));
    NamedFile::open(path).await.ok()
}

#[get("/get_pipeline_stage")]
pub async fn get_pipeline_stage() -> Option<RawHtml<String>> {
    let result = TEMPLATES
        .render("/templates/csv_mfr/pipeline_stage.html", &Context::default())
        .ok()
        .map(|value| (value))
        .unwrap_or(String::new());

    Some(RawHtml(result))
}
