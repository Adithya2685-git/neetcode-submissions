impl Solution {
    pub fn min_path_sum(mut grid: Vec<Vec<i32>>) -> i32 {

        for i in  0.. grid.len() {
            for j in 0..grid[0].len(){
                if i as i32-1<0 && j as i32 -1<0{
                    continue;
                }else if i as i32-1<0{ 
                    grid[i][j]+= grid[i][j-1]
                }else if j as i32-1 <0 {
                    grid[i][j]+= grid[i-1][j];
                }else{
                grid[i][j]+= grid[i][j-1].min(grid[i-1][j]);
                }
            }
        }

        let rows = grid.len();
        let cols = grid[0].len();
        grid[rows-1][cols-1]
    }
}
