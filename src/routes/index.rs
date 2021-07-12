use crate::models::answer::Answer;
use crate::models::imatrix::IMatrix;
use crate::use_cases::calculate_new_matrix::calculate_new_matrix;
use rocket::serde::json::Json;

#[post("/", format = "json", data = "<matrix>")]
pub fn index(matrix: Json<IMatrix>) -> Json<Answer> {
    let ans_vec = calculate_new_matrix(&matrix.matrix);

    Json(Answer {
        name: String::from("New Matrix"),
        matrix: ans_vec,
    })
}