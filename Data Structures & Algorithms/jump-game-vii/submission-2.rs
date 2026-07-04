impl Solution {
    pub fn can_reach(s: String, min_jump: i32, max_jump: i32) -> bool {
        // Convert to bytes for fast O(1) indexing (much faster than .chars())
        let s_bytes = s.as_bytes(); 
        
        // Quick optimization: If the last character is '1', we can never win
        if s_bytes[s_bytes.len() - 1] == b'1' {
            return false;
        }

        let min_j = min_jump as usize;
        let max_j = max_jump as usize;
        
        let mut dp = vec![false; s.len()];
        dp[0] = true;
        
        // This tracks how many `true` values are currently in our valid jump window
        let mut reachable_in_window = 0;

        for i in 1..s.len() {
            
            // 1. ADD to window: Does the index that just became reachable hold a 'true'?
            if i >= min_j && dp[i - min_j] {
                reachable_in_window += 1;
            }
            
            // 2. REMOVE from window: Did a 'true' just fall out of the back of our range?
            if i > max_j && dp[i - max_j - 1] {
                reachable_in_window -= 1;
            }

            // 3. Evaluate: If we have at least one valid jump source AND we are landing on a '0'
            if reachable_in_window > 0 && s_bytes[i] == b'0' {
                dp[i] = true;
            }
        }

        dp[s.len() - 1]
    }
}