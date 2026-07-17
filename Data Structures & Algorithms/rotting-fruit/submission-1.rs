use std::collections::VecDeque;
impl Solution {
    pub fn bfs(grid: &mut Vec<Vec<i32>> , mut q: VecDeque<(usize,usize)>)-> bool{
        let d = [(1,0), (-1,0), (0,1),(0,-1)];
        let mut changed = false;
        while let Some((i,j)) = q.pop_front(){
            for (dx,dy) in d.iter(){
                let x= i as i32+dx;
                let y= j as i32+dy;

                if x>=0 && x<grid.len() as i32 && y>=0 && y<grid[0].len() as i32 && grid[x as usize][y as usize]==1{
                    grid[x as usize][y as usize]=2;
                    changed =true;
                }
            }

        }

    changed
    }

    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut count= 0;
        let mut changed = true;
        let  mut q = VecDeque::new();
        while changed{ 
            for i in 0..grid.len(){
                for j in 0..grid[0].len(){
                    if grid[i][j] == 2{
                        q.push_back((i,j));
                    }
                }
            }
            changed = Self::bfs(&mut grid, q.clone());            
            
            if changed{
                count+=1;
            }
        }
            for row in &grid {
                for &cell in row {
                    if cell == 1 {
                        return -1; 
                    }
                }
            }
        count
    }
}