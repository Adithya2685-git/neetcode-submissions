use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut ans = Vec::with_capacity(2);

        let mut map:HashMap<i32,i32>= HashMap::with_capacity(nums.len());

        for (i,element) in nums.iter().enumerate(){
            let other = target- *element;
            let ind2 = i as i32;

            if map.contains_key(&other){
                let index= *map.get(&other).unwrap() as i32;
                ans.push(index.min(ind2));
                ans.push(index.max(ind2));
                return ans;
            }
            map.insert(*element,ind2);
        }
        ans
    }
}
