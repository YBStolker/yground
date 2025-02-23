mod hex_board;
mod hexagon;

use std::path::Path;

use hex_board::HexBoard;
use rocket::fs::relative;
use rocket::fs::NamedFile;
use rocket::get;
use rocket::response::content::RawHtml;
use rocket::routes;
use rocket::Route;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref BOARD: Option<HexBoard> = None;
}


#[get("/")]
pub async fn index() -> Option<NamedFile> {
    let path = Path::new(relative!("public/hexy/hexy.html"));
    NamedFile::open(path).await.ok()
}

#[get("/get_board?<size>")]
pub async fn get_board(size: u32) -> Option<RawHtml<String>> {
    if size == 0 {
        return None;
    }

    let board = HexBoard::new(size);

    Some(RawHtml(board.to_html()))
}

pub fn get_routes() -> Vec<Route> {
    routes![index, get_board]
}
