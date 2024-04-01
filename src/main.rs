pub mod csvmfr;

use std::path::Path;

use rocket::fs::relative;
use rocket::fs::FileServer;
use rocket::fs::NamedFile;
use rocket::get;
use rocket::routes;

#[get("/")]
async fn index() -> Option<NamedFile> {
    let path = Path::new(relative!("public/index.html"));
    NamedFile::open(path).await.ok()
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![index])
        .mount("/public", FileServer::from(relative!("public")))
        .mount("/template", FileServer::from(relative!("template")))
        .mount("/csvmfr", routes![csvmfr::index, csvmfr::add_stage])
        .launch()
        .await?;

    Ok(())
}
