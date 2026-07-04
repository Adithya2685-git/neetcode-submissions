impl Solution {
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut vec: Vec<Vec<i32>> = vec![vec![0 ; matrix.len()]; matrix[0].len()]; 

        for i in 0..matrix.len(){
            for j in 0..matrix[0].len(){
                vec[j][i] = matrix[i][j];
            }
        }

        vec
    }
}
