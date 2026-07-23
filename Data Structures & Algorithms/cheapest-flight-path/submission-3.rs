use std::collections::BinaryHeap;
use std::cmp:: Reverse;

impl Solution {
    pub fn bfs(n:i32 , graph: &Vec<Vec<(i32,i32)>>, src: i32, dst:i32, mut k: i32)-> i32{
        
        let mut pq = BinaryHeap::new();

        pq.push(Reverse((0,src,0)));
        let mut diff = vec![i32::MAX; n as usize];
        diff[src as usize]= 0;

        while !pq.is_empty(){ 
            
            let size = pq.len();
            for i in 0..size{
                let Reverse((wt, node, stops))= pq.pop().unwrap();

                if node ==dst{
                    return wt;
                }

                if stops>k || stops> diff[node as usize] {
                    continue;
                }
                for &(neigh,cost) in graph[node as usize].iter(){
                    diff[neigh as usize]= stops+1;
                    pq.push(Reverse((wt+ cost,neigh, stops+1)));
                }

            }

        }

        -1
    }
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let mut graph = vec![vec![]; n as usize];

        for flight in flights.iter(){
            let a = flight[0];
            let b = flight[1];
            let cost = flight[2];

            graph[a as usize].push((b,cost));

        }
        Self::bfs(n, &graph, src, dst, k )
    }
}
