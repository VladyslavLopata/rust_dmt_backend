use rocket::serde::json::Value;
use rocket::serde::json::json;
use rocket::serde::json::Json;
use crate::models::lab_first::LabFirst;
use crate::use_cases::calculate_lab_first::calculate_lab_first;


#[post("/api/lab1", data = "<matrix>")]
pub fn index(matrix: Json<LabFirst>) -> Json<Value> {
    let ans_vec = calculate_lab_first(&matrix.experts);

    Json(json!(ans_vec))
}