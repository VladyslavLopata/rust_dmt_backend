pub fn calculate_new_matrix(matrix: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut ans_vec = Vec::new();

    for i in matrix.iter() {
        ans_vec.push(Vec::new());
        for j in i.iter() {
            ans_vec.last_mut().unwrap().push(j+2);
        }
        
    }

    ans_vec
}