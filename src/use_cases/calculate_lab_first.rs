use crate::models::lab_first::Expert;

pub fn calculate_lab_first(experts: &Vec<Expert>) -> Vec<f32> {
    //number of experts
    let n_experts = experts.len();
    //number of alternatives
    let tln = experts[0].grades.len();

    // return an empty list if no experts or no grades
    if n_experts <= 0 || tln <= 0 {
        return vec![];
    }

    // sum of grades of a single expert
    let sum: Vec<i32> = experts.iter().map(|row| row.grades.iter().sum()).collect();

    let ans = experts
        .iter()
        .zip(sum)
        .fold(vec![0.0; tln], |previous, current| {
            previous
                .iter()
                .zip(current.0.grades.iter().map(|x| x.clone() as f32 / current.1 as f32 / n_experts as f32))
                .map(|zipped_i| zipped_i.0.clone() as f32 + zipped_i.1)
                .collect()
        });

    ans
}
