use rocket::serde::json::Json;
use rocket::serde::uuid::Uuid;
use rocket::*;

#[get("/")]
pub fn index() -> Json<&'static str> {
    Json("Welcome to the User route!")
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
pub fn info_user(id: Uuid) -> Json<String> {
    Json(format!("Info for user with id of {}", id))
}

#[put("/<id>")]
pub fn update_user(id: Uuid) -> Json<String> {
    Json(format!("Update info for user with id of {}", id))
}

#[delete("/<id>")]
pub fn delete_user(id: Uuid) -> Json<String> {
    Json(format!("Delete user with id of {}", id))
}
