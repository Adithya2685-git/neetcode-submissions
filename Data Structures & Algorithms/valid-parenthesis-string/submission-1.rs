impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        // Track the range of possible open '(' brackets
        let mut min_open = 0; 
        let mut max_open = 0; 

        for ch in s.chars() {
            if ch == '(' {
                min_open += 1;
                max_open += 1;
            } else if ch == ')' {
                min_open -= 1;
                max_open -= 1;
            } else { 
                // It's a '*'. 
                // It could be a ')' (lowering min) or a '(' (raising max) or empty (doing nothing)
                min_open -= 1; 
                max_open += 1; 
            }

            // If max_open falls below 0, it means we have too many ')' and not enough '(' or '*' to save it
            if max_open < 0 {
                return false;
            }

            // min_open can't be negative (a '*' can just act as an empty string instead of a ')')
            if min_open < 0 {
                min_open = 0;
            }
        }

        // If our minimum possible open brackets is 0, it's a valid string!
        min_open == 0
    }
}