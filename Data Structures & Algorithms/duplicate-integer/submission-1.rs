use std::collections::HashMap;

impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut map: HashMap<i32,bool> = HashMap::new();

        for element in nums.iter() {
            if map.contains_key(element){
                return true;
            }
            map.insert(*element, true);
        }
        false 
    }
}
