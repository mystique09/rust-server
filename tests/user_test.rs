use lazy_static;
use rocket::http::Status;
use rocket::local::blocking::Client;
use server::rocket_launcher;

#[test]
fn user_test() {
    let client = Client::tracked(rocket_launcher()).expect("Should be a valid rocket instance");
    let response = client.get("/user/").dispatch();
    assert_eq!(response.status(), Status::Ok);
}
