use rocket::get;

#[get("/test_static")]
pub fn test_static() {}

#[get("/test_fn")]
pub fn test_fn() {}
