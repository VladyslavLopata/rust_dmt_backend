use serde::Deserialize;

#[derive(Deserialize)]
pub struct IMatrix {
    pub matrix: Vec<Vec<i32>>,
}
