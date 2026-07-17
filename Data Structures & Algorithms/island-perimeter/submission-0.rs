use std::collections::VecDeque;
impl Solution {

    pub fn bfs(grid: &Vec<Vec<i32>> , visited: &mut Vec<Vec<bool>>, i: i32, j: i32)-> i32{

        let mut q= VecDeque::new();

        q.push_back((i,j));
        visited[i as usize][j as usize]= true;
        let d= [(1,0),(-1, 0),(0,1),(0,-1)];
        let mut perimeter =0;

        while let Some((x,y))= q.pop_front(){

            let mut walls =0;
            for &(x1,y1) in d.iter(){
                if x+x1 >=0 && x+x1< grid.len() as i32 && y+y1>=0 && y+y1< grid[0].len() as i32 && grid[(x+x1) as usize][(y+y1) as usize] ==1{
                   if !visited[(x+x1)as usize][(y+y1) as usize]{
                        q.push_back((x+x1, y+y1));
                        visited[(x+x1) as usize][(y+y1) as usize]= true;
                   }

                }else{
                    walls +=1;
                } 
            }

            perimeter+= walls;
        }

    perimeter
    }


    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let mut visited = vec![vec![false; grid[0].len()];grid.len()];

        let mut result = 0;
        for i in 0.. grid.len(){
            for j in 0..grid[0].len(){
                if !visited[i][j] && grid[i][j]== 1{
                    result =Self::bfs(&grid,&mut visited, i as i32, j as i32);

                }
            }
        }

        result
    }
}
