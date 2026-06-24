use std::collections::VecDeque;

impl Solution {

    pub fn bfs(root: i32, graph: &Vec<Vec<i32>>, visited:&mut Vec<bool>) {
        
        let mut queue = VecDeque::new();

        queue.push_back(root);
        visited[root as usize] = true;

        while let Some(node) = queue.pop_front(){

            for neighbors in &graph[node as usize ]{
                if !visited[*neighbors as usize]{
                queue.push_back(*neighbors);
                visited[*neighbors as usize] = true;        
                }
            }
        }


    }

    pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {

        let mut graph: Vec<Vec<i32>> = vec![vec![]; n as usize];
        
        for edge in edges.iter(){
            let a = edge[0] ; 
            let b = edge[1] ;

            graph[a as usize].push(b);
            graph[b as usize].push(a);
        }

        let mut visited: Vec<bool> = vec![false; n as usize];
        let mut count =0;
        for node in 0..n{
            if !visited[node as usize]{
                Self::bfs(node,&graph, &mut visited);
                count+=1;
            }
        }
        count
    }
}
