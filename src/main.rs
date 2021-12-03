#[macro_use]
extern crate rocket;

// mod models;
// use models::{user};

#[get("/")]
fn index() -> &'static str {
    "Hello, World!"
}

#[get("/user/<user>")]
fn user(user: &str) -> String {
    format!("Hello {}", user)
}

#[rocket::main]
async fn main() {
    match rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![user])
        .launch()
        .await {
            Ok(()) => println!("Compiled successfully"),
            Err(_) => println!("Something went wrong!")
        }
}