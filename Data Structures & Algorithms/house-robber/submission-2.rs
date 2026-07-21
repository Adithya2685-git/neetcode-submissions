impl Solution {
    pub fn recursion(index: usize, nums: &Vec<i32>, dp : &mut Vec<i32>)-> i32{
        if index >=nums.len(){
            return 0;
        }

        if dp[index]!= -1{return dp[index];}

        let l = nums[index]+ Self::recursion(index+2, nums,dp);
        let r = Self::recursion(index+1, nums,dp);

        dp[index] = l.max(r);
        return dp[index];
    }

    pub fn rob(nums: Vec<i32>) -> i32 {
        
        let mut dp = vec![-1; nums.len()];
        Self::recursion(0, &nums, &mut dp);
        dp[0]
    }
}
