#[macro_use]
extern crate rocket;

mod models;
use crate::models::answer::Answer;
use crate::models::imatrix::IMatrix;
use rocket::serde::json::Json;

#[post("/", format = "json", data = "<matrix>")]
fn hello(matrix: Json<IMatrix>) -> Json<Answer> {
    let mut ans_vec = Vec::new();

    for i in matrix.matrix.iter() {
        ans_vec.push(Vec::new());
        for j in i.iter() {
            ans_vec.last_mut().unwrap().push(j+2);
        }
        
    }

    Json(Answer {
        name: String::from("AAAA"),
        matrix: ans_vec,
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello])
}
