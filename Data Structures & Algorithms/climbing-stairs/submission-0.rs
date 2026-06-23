impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let n = n as usize;
        let mut dp:Vec<i32> = vec![0; n+1];
        dp[0] = 1;
        dp[1] = 2;
        for i in 2..n{
            dp[i] += dp[i-2] +dp[i-1];
        }
        dp[n-1]
    }
}
