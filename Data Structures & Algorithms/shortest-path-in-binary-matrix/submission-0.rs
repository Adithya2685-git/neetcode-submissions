use std::collections::{VecDeque };
impl Solution {
    pub fn bfs(grid: &mut Vec<Vec< i32>>)-> i32{

        let mut q = VecDeque::new();

        let d = [(0,1), (0,-1),(1,0),(-1,0),(-1,-1),(1,1),(-1,1),(1,-1)];
        q.push_back((0,0));
        grid[0][0]= 1;
        while let Some((x,y)) = q.pop_front(){

            if x==grid.len()-1 && y == grid[0].len()-1{
                return grid[x][y];
            }

            for dx in d.iter(){
                let i= x as i32+dx.0;
                let j= y as i32+dx.1;

                if i>=0 && i<grid.len() as i32 && j>=0 && j< grid[0].len() as i32 &&grid[i as usize][j as usize]==0{

                    q.push_back((i as usize,j as usize));
                    grid[i as usize][j as usize]=grid[x][y]+1;

                }
            }
        }
        -1
    }

    pub fn shortest_path_binary_matrix(mut grid: Vec<Vec<i32>>) -> i32 {
        if grid[0][0]== 1 || grid[grid.len()-1][grid[0].len()-1]==1{
            return -1;
        }

        Self::bfs(&mut grid)

    }
}