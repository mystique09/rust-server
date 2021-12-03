use rocket::*;

#[get("/")]
pub fn index() -> &'static str {
  "Welcome to the message route!"
}

#[get("/all")]
pub fn all_message() -> &'static str {
  "Should return all user"
}

#[post("/new", format = "json", data = "<message>")]
pub fn new_message(message: &str) -> &'static str {
  println!("Message Data: {}", &message);
  "Add new message"
}