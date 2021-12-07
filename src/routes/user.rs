use rocket::*;
use rocket::{
  request::Request,
  response::{self, Response, Responder},
  serde::json::Json
};
use crate::serde::{
  Serialize,
  Deserialize
};
use rocket::serde::uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
  pub status: String,
  pub message: String
}

impl<'r> Responder<'r, 'static> for Data {
  fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
    Response::build()
    //.raw_header("X-Data-Status", self.status)
    .ok()
  }
}

impl Data {
  pub fn ok(message: &str) -> Self {
    Data {
      status: "Success".to_string(),
      message: message.to_string(),
    }
  }
  
  pub fn err(message: &str) -> Self {
    Data {
      status: "Success".to_string(),
      message: message.to_string(),
    }
  }
}

#[get("/")]
pub fn index() -> Json<Data> {
  Json(Data::ok("Welcome to the User route!"))
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

#[get("/<id>")]
pub fn info_user(id: Uuid) -> Json<Data> {
  Json(Data::ok(&* format!("Info for user {}", id)))
}

#[put("/<id>")]
pub fn update_user(id: Uuid) -> Json<Data> {
  Json(Data::ok(&* format!("Update info for user {}", id)))
}

#[delete("/<id>")]
pub fn delete_user(id: Uuid) -> Json<Data> {
  Json(Data::ok(&* format!("Delete user with id {}", id)))
}