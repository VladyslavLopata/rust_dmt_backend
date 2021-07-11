use serde::Serialize;

#[derive(Serialize)]
pub struct Answer {
    pub name: String,
    pub matrix: Vec<Vec<i32>>,
}
