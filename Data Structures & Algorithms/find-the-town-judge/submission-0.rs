impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {

        let mut graph1 = vec![vec![]; (n+1) as usize];
        let mut graph2 = vec![vec![]; (n+1) as usize];

        for edge in trust.iter(){
            let a = edge[0];
            let b = edge[1];

            graph1[b as usize].push(a);  
            graph2[a as usize].push(b);          
        }

        for (i,lists) in graph1.iter().enumerate(){
            if lists.len() == n as usize-1 && i!= 0 && graph2[i].is_empty(){
                return i as i32;
            }
        } 

        -1
    }
}
