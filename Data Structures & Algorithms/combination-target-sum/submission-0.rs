use std::collections::HashSet;
impl Solution {
    pub fn recursion(nums: &Vec<i32>, i: usize, v: &mut Vec<i32>, mut sum: i32 , target: i32, set: &mut Vec<Vec<i32>>){

        if sum == target{
            set.push(v.clone());
            return;
        }
        if i == nums.len()|| sum>target{
            return ;
        }

        v.push(nums[i]);
        sum+= nums[i];

        Self::recursion(nums, i, v, sum, target, set);

        v.pop();
        sum-= nums[i];
        Self::recursion(nums, i+1, v, sum, target, set);

    }
    
    pub fn combination_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut v= Vec::new();
        let mut result = Vec::new();
        Self::recursion(&nums, 0, &mut v, 0,target, &mut result );
        result
    }
}
