use crate::lazy_static::lazy_static;
use rocket::local::blocking::Client;
use server::rocket_launcher;

pub fn setup_test() -> &'static Client {
  lazy_static! {
    static ref CLIENT: Client = Client::tracked(rocket_launcher()).expect("Valid rocket instance!");
  }
  &*CLIENT
}