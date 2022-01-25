use rocket::*;
pub mod guards;
pub mod message;
pub mod user;

#[get("/")]
pub fn index_route() -> &'static str {
    "Hello, World!"
}
