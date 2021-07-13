#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

use rocket::http::Method;
use rocket::{Build, Rocket};
use rocket_cors::{AllowedHeaders, AllowedOrigins};

mod models;
mod routes;
mod use_cases;

pub fn rocket_run() -> Rocket<Build> {
        let cors = rocket_cors::CorsOptions {
                allowed_origins: AllowedOrigins::all(),
                allowed_methods: vec![Method::Post].into_iter().map(From::from).collect(),
                allowed_headers: AllowedHeaders::some(&["Authorization", "Accept", "Content-type"]),
                allow_credentials: true,
                expose_headers: ["Content-Type", "X-Custom", "Content-Length"]
                        .iter()
                        .map(ToString::to_string)
                        .collect(),
                ..Default::default()
        }
        .to_cors()
        .unwrap();
        rocket::build()
                .mount("/", routes![routes::index::index])
                .mount("/", rocket_cors::catch_all_options_routes())
                .attach(cors.clone())
                .manage(cors)
}
