use lazy_static;
use rocket::http::Status;
use server::rocket_launcher;
mod common;
use common::setup_test;

#[test]
fn user_test() {
    let client = setup_test();
    let response = client.get("/user/").dispatch();
    assert_eq!(response.status(), Status::Ok);
}
