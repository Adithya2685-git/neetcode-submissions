use std::collections::VecDeque;
impl Solution {
    pub fn bfs(grid: &Vec<Vec<i32>>, visited: &mut Vec<Vec<bool>>, q: &mut VecDeque<(usize, usize)>)-> (bool,i32){
        let d = [(1,0), (-1,0), (0,1), (0,-1)];
        let mut counts= 0;
        let mut is_border= false;
        while let Some((i,j)) = q.pop_front(){
            counts+=1;
            if i==0 || i== grid.len()-1 || j==0 || j== grid[0].len()-1{
                is_border= true;
            }
            for &(dx,dy) in d.iter(){
                let x= i as i32+ dx;
                let y= j as i32+dy;
                if x>=0 && x<grid.len() as i32 && y>=0 && y< grid[0].len() as i32 && grid[x as usize][y as usize] ==1 && !visited[x as usize][y as usize]{
                    visited[x as usize][y as usize]= true;
                    q.push_back((x as usize,y as usize));
                }
            }
        }

        (is_border, counts)
    }

    pub fn num_enclaves(grid: Vec<Vec<i32>>) -> i32 {
        let mut q = VecDeque::new();
        let mut visited = vec![vec![false;grid[0].len()];grid.len()];
        let mut total = 0; 
        for i in 0..grid.len(){
            for j in 0..grid[0].len(){
                if !visited[i][j] && grid[i][j]==1 {
                        q.clear();
                        q.push_back((i, j));
                        visited[i][j] = true;

                    if i==0 || i== grid.len()-1 || j==0 || j==grid[0].len()-1{
                        Self::bfs(&grid, &mut visited, &mut q);
                    }else{

                        let (is_border,current)=  Self::bfs(&grid, &mut visited, &mut q);
                        if !is_border{
                            total +=current;
                        }
                    }

                }
            }
        }

        total
    }
}
