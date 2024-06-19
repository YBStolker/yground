pub mod csv_mfr;
pub mod test;

use std::path::Path;

use rocket::fs::relative;
use rocket::fs::FileServer;
use rocket::fs::NamedFile;
use rocket::get;
use rocket::response::content::RawHtml;
use rocket::routes;
use tera::Context;
use tera::Tera;

use lazy_static::lazy_static;

#[get("/")]
async fn index() -> Option<NamedFile> {
    let path = Path::new(relative!("public/index.html"));
    NamedFile::open(path).await.ok()
}

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let tera = Tera::new("templates/**/*.html").expect("Could not create Tera object.");
        tera
    };
}

#[get("/navbar/<active>")]
async fn navbar(active: Option<&str>) -> Option<RawHtml<String>> {
    let active = active.unwrap_or("home");

    let mut context = Context::new();
    context.insert("home", "");
    context.insert("csv_mfr", "");

    if context.contains_key(active) {
        context.insert(active, "active")
    }

    let result = RawHtml(TEMPLATES.render("navbar.html", &context).unwrap());

    Some(result)
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![index, navbar])
        .mount("/public", FileServer::from(relative!("public")))
        .mount("/test", routes![test::test_static, test::test_fn])
        .mount(
            "/csv_mfr",
            routes![csv_mfr::index, csv_mfr::get_pipeline_stage],
        )
        .launch()
        .await?;

    Ok(())
}
