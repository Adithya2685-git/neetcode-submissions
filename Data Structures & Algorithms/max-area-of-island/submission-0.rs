use std::collections::VecDeque;

impl Solution {

    pub fn bfs(grid: &Vec<Vec<i32>>,visited: &mut Vec<Vec<bool>>, i: usize, j: usize)-> i32{

        let mut q = VecDeque::new();

        q.push_back((i,j));
        visited[i][j]= true;

        let mut count = 1;
        while let Some((i,j)) = q.pop_front(){

            if i>0 && !visited[i-1][j]{
                visited[i-1][j]= true;
                if grid[i-1][j] == 1{
                    q.push_back((i-1,j));
                    count+=1;
                }

            }
            if i < grid.len()-1&& !visited[i+1][j]{
                visited[i+1][j]= true;
                if grid[i+1][j] == 1{
                    q.push_back((i+1,j));
                    count+=1;
                }
            }
            if j < grid[0].len()-1 && !visited[i][j+1]{
                visited[i][j+1]= true;
                if grid[i][j+1] == 1{
                    q.push_back((i,j+1));
                    count+=1;
                }
            }

            if j>0 && !visited[i][j-1]{
                visited[i][j-1]= true;
                if grid[i][j-1] == 1{
                    q.push_back((i,j-1));
                    count+=1;
                }
            }

        }
        count
    }


    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {

        let mut visited = vec![vec![false;grid[0].len()];grid.len()];
        let mut max = 0;
        for i in 0..grid.len(){
            for j in 0..grid[0].len(){
                if !visited[i][j] && grid[i][j]!= 0{
                    max = max.max(Self::bfs(&grid, &mut visited, i, j));

                }
            }
        }

        max
    }
}
