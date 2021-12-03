use rocket::*;

pub mod user;
pub mod message;

#[get("/")]
pub fn index_route() -> &'static str {
  "Hello, World!"
}