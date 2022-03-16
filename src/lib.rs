pub mod models;
pub mod routes;
pub mod schema;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use rocket::*;
use routes::{index_route, message as message_route, user as user_route};
use std::env;
use std::sync::{Arc, Mutex};

pub struct Db {
    pub conn: Arc<Mutex<PgConnection>>,
}

pub fn establish_dbconn() -> PgConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE URL ENV MUST BE SET!");

    PgConnection::establish(&db_url).unwrap()
}

pub fn rocket_launcher() -> Rocket<Build> {
    let conn: Arc<Mutex<PgConnection>> = Arc::new(Mutex::new(establish_dbconn()));

    rocket::build()
        .mount("/", routes![index_route])
        .mount(
            "/user",
            routes![
                user_route::index,
                user_route::all_user,
                user_route::new_user,
                user_route::info_user,
                user_route::update_user,
                user_route::delete_user,
            ],
        )
        .mount(
            "/message",
            routes![
                message_route::index,
                message_route::all_message,
                message_route::new_message
            ],
        )
        .manage(Db { conn })
}
