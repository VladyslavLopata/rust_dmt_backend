#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket};

mod cors_utils;
mod models;
mod routes;
mod use_cases;

pub fn rocket_run() -> Rocket<Build> {
        let cors = cors_utils::get_default_cors();
        rocket::build()
                .mount("/", routes![routes::index::index])
                .mount("/", rocket_cors::catch_all_options_routes())
                .attach(cors.clone())
                .manage(cors)
}
