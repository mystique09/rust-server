use diesel::pg::PgConnection;
use rocket::*;
mod models;
use models::*;
pub mod routes;

use routes::{index_route, message as message_route, user as user_route};

pub fn rocket_launcher() -> Rocket<Build> {
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
}
