use serde::Deserialize;

#[derive(Deserialize)]
pub struct LabFirst {
    pub experts: Vec<Expert> ,
}

#[derive(Deserialize)]
pub struct Expert {
    pub grades: Vec<i32>,
}
