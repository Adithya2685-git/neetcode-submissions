use std::collections:: VecDeque;
impl Solution {
    pub fn bfs(grid:&mut Vec<Vec<i32>>,i:usize,j: usize ){
        
        let mut q = VecDeque::new();

        q.push_back((i,j));

        while let Some((x,y)) = q.pop_front(){
            if x > 0 && grid[x-1][y] != -1 && grid[x][y] + 1 < grid[x-1][y] {
                grid[x-1][y] = grid[x][y] + 1;
                q.push_back((x-1, y));
            }

            if y>0 && grid[x][y-1] != -1 && grid[x][y] +1 < grid[x][y-1]{
                grid[x][y-1]= grid[x][y]+1;
                q.push_back((x,y-1));
            }
            if x+1<grid.len() && grid[x+1][y] != -1 && grid[x+1][y]>grid[x][y]+1{
                grid[x+1][y]= grid[x][y]+ 1; 
                q.push_back((x+1,y));
            }
            if y+1<grid[0].len() && grid[x][y+1] != -1 && grid[x][y+1] > grid[x][y] +1{
                grid[x][y+1]= grid[x][y]+1; 
                q.push_back((x,y+1));
            }

        }
    }

    pub fn islands_and_treasure(grid: &mut Vec<Vec<i32>>) {

        for i in 0..grid.len(){
            for j in 0..grid[0].len(){
                if grid[i][j]==0{
                    Self::bfs(grid,i,j);
                }
            }
        }

    }
}
