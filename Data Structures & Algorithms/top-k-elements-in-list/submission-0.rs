use std::collections::HashMap;
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {

        let mut map: HashMap<i32,i32> = HashMap::new();
        
        for elements in nums.iter(){
            let val =map.entry(*elements).or_insert(0);
            *val+=1;
        }

        let mut frequencies = vec![vec![]; nums.len()+1];
        let mut result = Vec::new();
        
        for (element, &freq) in map.iter(){
            frequencies[freq as usize].push(element);
        }

        
        for buckets in frequencies.iter().rev(){
            for e in buckets{
                result.push(**e);
                if result.len()== k as usize{
                    return result
                }
            }
        }
        result
    }
}
