use rocket::*;

#[get("/")]
pub fn index() -> &'static str {
  "Welcome to the User route!"
}

#[get("/all")]
pub fn all_user() -> &'static str {
  "Should fetch all user"
}

#[post("/new", format = "json", data = "<user>")]
pub fn new_user(user: &str) -> &'static str {
  println!("User Data: {}", &user);
  "Add new user"
}