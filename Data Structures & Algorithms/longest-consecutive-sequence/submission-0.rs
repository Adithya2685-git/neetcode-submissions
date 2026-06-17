use std::collections::{HashSet};
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {

        let set: HashSet<i32> = nums.into_iter().collect();

        let mut global_max = 0;
        for element in set.iter(){ 
            let mut count = 1;

            let Some(_) = set.get(&(*element -1)) else{
                while let Some(_) = set.get(&(*element+ count)){count+=1};
                global_max = global_max.max(count);
            
            
                continue;
            };

        } 

        global_max
        
    }
}
