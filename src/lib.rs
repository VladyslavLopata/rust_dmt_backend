#[macro_use]
extern crate rocket;

use rocket::Build;
use rocket::Rocket;

mod routes;
mod models;
mod use_cases;

pub fn rocket_run() -> Rocket<Build> {
    rocket::build().mount("/", routes![routes::index::index])
}