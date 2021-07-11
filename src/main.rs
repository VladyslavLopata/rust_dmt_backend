#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;


use rocket_contrib::json::Json;
use serde::Serialize;


#[derive(Serialize)]
struct Answer {
    name: String,
    matrix: Vec<Vec<i32>>
}

#[get("/")]
fn index() -> Json<Answer> {
    Json(Answer{
        name: String::from("AAA"),
        matrix: vec!(vec!(1,2,3))
    })
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}