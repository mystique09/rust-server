extern crate rocket;

use server::rocket_launcher;

#[rocket::main]
async fn main() {
    match rocket_launcher().launch().await {
      Ok(()) => println!("Server Started!"),
      Err(_) => println!("Something went wrong!")
    }
}