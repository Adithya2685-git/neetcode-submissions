impl Solution {
    pub fn recursion(index: usize, nums: &Vec<i32>, dp : &mut Vec<i32>)-> i32{
        if index==0{
            return nums[0];
        }

        if dp[index]!= -1{return dp[index];}
        let mut l =nums[index];
        if index>=2{l = nums[index]+ Self::recursion(index-2, nums,dp);}

        let mut r= 0;
        if index>=1{ r = Self::recursion(index-1, nums,dp);}

        dp[index] = l.max(r);
        return dp[index];
    }

    pub fn rob(nums: Vec<i32>) -> i32 {
        
        let mut dp = vec![-1; nums.len()];

        Self::recursion(nums.len()-1, &nums, &mut dp)
    }
}
