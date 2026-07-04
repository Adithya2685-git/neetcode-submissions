impl Solution {
    pub fn can_reach(s: String, min_jump: i32, max_jump: i32) -> bool {

        let mut dp = vec![0; s.len()];

        dp[0]= 1;

        for (i,chars) in s.chars().enumerate() {

            if chars== '1' {
                dp[i] = 0;
                continue;

            }else{
                
                for jumps in (min_jump as usize)..= (max_jump as usize){
                    if i>=jumps{
                        dp[i] += dp[i-jumps];
                    }
                }

            }
        }

        if dp[s.len()-1] > 0{
            true
        }else{
            false
        }

    }
}
