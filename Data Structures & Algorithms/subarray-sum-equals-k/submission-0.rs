use std::collections::HashMap;
impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut map= HashMap::new();
        let mut sums = vec![0;nums.len()];
        sums[0]= nums[0];
        for i in 1..nums.len(){
            sums[i]= sums[i-1]+ nums[i];
        }

        let mut result =0;
        map.insert(0,1);
        for i in 0..nums.len(){
            let targeti= sums[i]-k;

            if let Some(val)= map.get(&targeti){
                result+= val; 
            }

            *map.entry(sums[i]).or_insert(0)+=1;
        }
        result
    }
}
