use rocket::*;

mod models;
mod routes;
use routes::{index_route, message, user};

pub fn rocket_launcher() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![index_route])
        .mount(
            "/user",
            routes![
                user::index,
                user::all_user,
                user::new_user,
                user::info_user,
                user::update_user,
                user::delete_user,
            ],
        )
        .mount(
            "/message",
            routes![message::index, message::all_message, message::new_message],
        )
}
