use std::collections::HashSet;

impl Solution {
    pub fn dfs(graph: &Vec<Vec<i32>>) -> Vec<i32> {
        let mut visited: Vec<bool> = vec![false; graph.len()+ 1];
        
        // parent array to trace our path back when we find a cycle
        let mut parent: Vec<i32> = vec![-1; graph.len()+1 ];

        // Stack now stores: (current_node, parent_node)
        let mut q: Vec<(i32, i32)> = Vec::new();

        q.push((1, -1));

        while let Some((node, p)) = q.pop() {
            
            // If we pop a node and it's already visited, WE FOUND THE CYCLE!
            if visited[node as usize] {
                let mut cyclelist: Vec<i32> = Vec::new();
                cyclelist.push(node);
                
                // Trace backwards through the parents to extract the exact cycle
                let mut curr = p;
                while curr != node && curr != -1 {
                    cyclelist.push(curr);
                    curr = parent[curr as usize];
                }
                return cyclelist;
            }

            // Mark as visited and record where we came from
            visited[node as usize] = true;
            parent[node as usize] = p;

            // Push neighbors to the stack
            for &neigh in graph[node as usize].iter() {
                
                // CRITICAL FIX: Do not look backwards at the node we just came from!
                if neigh != p {
                    q.push((neigh, node));
                }
            }
        }
        
        vec![]
    }
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        

        let mut graph: Vec<Vec<i32>> = vec![vec![]; edges.len() +1];

        for edge in edges.iter(){
            let a = edge[0];
            let b = edge[1];

            graph[b as usize].push(a);
            graph[a as usize].push(b);
        }

        let list = Self::dfs(&graph);
        let mut cycgraph = HashSet::new();

        for i in 0..(list.len()-1){
            let a = list[i];
            let b = list[i+1];

            cycgraph.insert(vec![a.min(b),a.max(b)]);
        }
        let a = list[list.len()-1];
        let b = list[0];
        cycgraph.insert(vec![a.min(b),a.max(b)]);

        for edge in edges.iter().rev(){
            if cycgraph.contains(edge){
                return edge.clone()
            }
        }
        edges[0].clone()
    }
}
