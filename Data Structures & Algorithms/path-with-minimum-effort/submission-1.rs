use std::collections::BinaryHeap; 
use std::cmp::Reverse;

impl Solution {
    pub fn bfs(heights: & Vec<Vec<i32>>)-> i32{
        let mut pq = BinaryHeap::new();
        let d = [(0,1), (0,-1), (1,0) , (-1,0)];
        let mut diff = vec![vec![i32::MAX;heights[0].len()] ;heights.len()];

        pq.push(Reverse((0i32,0usize,0usize)));
        diff[0][0]= 0;
        while let Some(Reverse((wt,x,y)))= pq.pop(){

            if x == heights.len()-1 &&  y== heights[0].len()-1{
                return diff[x][y];
            }
            for dx in d.iter(){
                let i = x as i32 + dx.0;
                let j = y as i32 + dx.1;

                if i>=0 && i<heights.len() as i32 && j>=0 && j< heights[0].len() as i32{

                    let (i,j) = (i as usize, j as usize);
                    let new= (heights[i][j].abs_diff(heights[x][y]) as i32).max(wt); 
                    if  new< diff[i][j]{
                        diff[i][j]=new ;
                        pq.push( Reverse(( new, i, j)));
                    }
                }

            }
        } 
        -1
    }


    pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
        Self::bfs(& heights)
    }
}
