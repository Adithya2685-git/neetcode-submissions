impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {

        let mut ans = String::new();

        let first_str = strs[0].as_bytes();
        for (i,chars) in first_str.iter().enumerate(){
            

            for strings in strs.iter(){

                let other_bytes = strings.as_bytes();
                
       
                if i == other_bytes.len() || other_bytes[i] != *chars {
                    return ans;
                }
            }
            ans.push(*chars as char);
        }

        ans
    }
}
