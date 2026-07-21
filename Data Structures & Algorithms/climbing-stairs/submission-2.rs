impl Solution {
    pub fn recursion(i: i32,dp: &mut Vec< i32>)-> i32{
        if i ==0 || i==1{
            return  1;
        }
        if dp[i as usize]!= -1{ return dp[i as usize];}

        dp[i as usize]= Self::recursion(i-1,dp) + Self::recursion(i-2,dp);
        return dp[i as usize];
    }

    pub fn climb_stairs(n: i32) -> i32 {
        let mut dp = vec![-1; n as usize+1];
        dp[0] =1;
        dp[1]= 1;
        Self::recursion(n,&mut dp) 
    }
}
