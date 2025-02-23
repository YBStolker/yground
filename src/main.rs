mod csv_mfr;
mod hexy;
mod util;

use std::path::Path;

use rocket::fs::relative;
use rocket::fs::FileServer;
use rocket::fs::NamedFile;
use rocket::get;
use rocket::response::content::RawHtml;
use rocket::response::Redirect;
use rocket::routes;
use tera::Context;
use tera::Tera;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref TEMPLATES: Tera = Tera::new("**/*.html").expect("Could not create Tera object.");
}

#[get("/")]
async fn index() -> Option<NamedFile> {
    let path = Path::new(relative!("public/index.html"));
    NamedFile::open(path).await.ok()
}

#[get("/favicon.ico")]
async fn favicon() -> Redirect {
    Redirect::to("/public/favicon.ico")
}

#[get("/navbar/<active>")]
async fn navbar(active: Option<&str>) -> Option<RawHtml<String>> {
    let active = active.unwrap_or("home");

    let mut context = Context::new();
    context.insert("home", "");
    context.insert("csv_mfr", "");
    context.insert("hexy", "");

    if context.contains_key(active) {
        context.insert(active, "active")
    }

    let result = TEMPLATES.render("templates/navbar.html", &context).unwrap();

    Some(RawHtml(result))
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![index, navbar, favicon])
        .mount("/public", FileServer::from(relative!("public")))
        .mount("/csv_mfr", csv_mfr::get_routes())
        .mount("/hexy", hexy::get_routes())
        .launch()
        .await?;

    Ok(())
}
