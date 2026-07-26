impl Solution {
    pub fn warshall(graph: &mut Vec<Vec<bool>> , queries: &Vec<Vec<i32>>, n: usize)->Vec<bool>{
        let mut result = vec![false;queries.len()];

        for d in 0..n{
            for i in 0..n{
                for j in 0..n{

                    graph[i][j]= graph[i][j]|| ( graph[i][d] && graph[d][j]);
                }
            } 
        }
        for (i,q) in queries.iter().enumerate(){
            let u = q[0];
            let v = q[1];

            if graph[u as usize][v as usize]{
                result[i]= true;
            }
        }
        result
    }

    pub fn check_if_prerequisite(num_courses: i32, prerequisites: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        
        let mut graph = vec![vec![false; num_courses as usize];num_courses as usize];
        for i in 0..num_courses as usize{
            graph[i][i]= true;
        }

        for req in prerequisites.iter(){
            let a= req[0];
            let b = req[1];

            graph[a as usize][b as usize]= true;
        }
        Self::warshall(&mut graph, &queries,num_courses as usize)

    }
}