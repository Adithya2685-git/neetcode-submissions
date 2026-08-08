use std::collections::HashMap;
impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {

        let mut map: HashMap<i32, Vec<usize>> = HashMap::new();
        for i in 0..nums.len(){
            let v =map.entry(nums[i]).or_insert(Vec::new());
            v.push(i);
        }


        for (_,v) in map.iter(){
            for i in 1..v.len(){
                if v[i-1].abs_diff(v[i])<=k as usize{
                    return true;
                }
            }
        }
        false
    }
}
