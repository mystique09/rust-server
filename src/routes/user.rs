use crate::models::user::{UserModel, CreateUser};
use crate::routes::guards::auth_guard::AuthGuard;
use crate::Db;
use rocket::http::{Cookie, CookieJar};
use rocket::serde::json::{Value, json, Json};
use rocket::serde::uuid::Uuid;
use rocket::State;
use rocket::*;

#[get("/")]
pub fn index(jar: &CookieJar<'_>) -> Json<&'static str> {
    jar.add_private(Cookie::new("auth_token", "jsjsjsjsjsjs"));
    Json("Welcome to the User route!")
}

#[get("/all")]
pub fn all_user(_auth: AuthGuard, db: &State<Db>) -> Json<Value> {
    let conn = db.conn.lock().unwrap();
    let results = UserModel::all_user(&conn);
    Json(json!{ results })
}

#[post("/new", format = "json", data = "<user>")]
pub fn new_user(user: &str) -> &'static str {
    "Add new user"
}

#[get("/<id>")]
pub fn info_user(id: i32) -> Json<String> {
    Json(format!("Info for user with id of {}", id))
}

#[put("/<id>")]
pub fn update_user(id: i32) -> Json<String> {
    Json(format!("Update info for user with id of {}", id))
}

#[delete("/<id>")]
pub fn delete_user(id: Uuid) -> Json<String> {
    Json(format!("Delete user with id of {}", id))
}
