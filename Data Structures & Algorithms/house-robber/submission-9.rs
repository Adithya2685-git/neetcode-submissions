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

        dp[0]= nums[0];
        for i in 1..nums.len(){

            let mut take = nums[i];
            if i>=2{take +=dp[i-2];}
            let mut skip= dp[i-1];

            dp[i]= take.max(skip);
        }
        dp[nums.len()-1]
    }
}
