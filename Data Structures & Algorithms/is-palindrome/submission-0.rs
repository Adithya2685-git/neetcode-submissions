impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut s_lower = s.to_lowercase();
        s_lower.retain(|c| c.is_alphanumeric());
        let s_rev: String = s_lower.chars().rev().collect();
        println!("{s_lower}");
        if s_rev == s_lower{
            true
        }
        else{
            false
        }
    }
}
