use rocket::http::Method;
use rocket_cors::{AllowedHeaders, AllowedOrigins};

pub fn get_default_cors() -> rocket_cors::Cors {
    rocket_cors::CorsOptions {
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
    .unwrap()
}
