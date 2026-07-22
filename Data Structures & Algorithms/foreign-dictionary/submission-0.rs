use std::collections::{HashMap, VecDeque};
impl Solution {

    pub fn topo(graph: &HashMap<char, Vec<char>>) -> String {

        let mut q = VecDeque::new();
        let mut indeg: HashMap<char, i32> = HashMap::new();

        for &node in graph.keys(){
            indeg.entry(node).or_insert(0);
        }

        for (_, v) in graph.iter(){
            for &neigh in v.iter(){
                *indeg.entry(neigh).or_insert(0) += 1;
            }
        }

        for (&node, &deg) in indeg.iter(){
            if deg == 0{
                q.push_back(node);
            }
        }

        let mut result = String::new();
        while let Some(node) = q.pop_front(){
            result.push(node);
            for &neigh in graph.get(&node).unwrap().iter(){
                let d = indeg.entry(neigh).or_insert(0);
                *d -= 1;
                if *d == 0{
                    q.push_back(neigh);
                }
            }
        }
        result
    }

    pub fn foreign_dictionary(words: Vec<String>) -> String {
        let mut graph: HashMap<char, Vec<char>> = HashMap::new();

        for s in words.iter(){
            for c in s.chars(){
                graph.entry(c).or_insert(vec![]);
            }
        }
        for i in 0..words.len() - 1 {
            let s1 = words[i].as_bytes();
            let s2 = words[i + 1].as_bytes();

            let len = s1.len().min(s2.len());
            let mut found = false;

            for j in 0..len {
                if s1[j] != s2[j] {
                    graph.get_mut(&(s1[j] as char)).unwrap().push(s2[j] as char);
                    found = true;
                    break;
                }
            }

            if !found && s1.len() > s2.len() {
                return String::new();
            }
        }

        let result = Self::topo(&graph);
        if result.len()!= graph.len(){
            String::new()
        }else{
            result
        }
    }
}