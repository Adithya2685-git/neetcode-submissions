use std::collections::HashMap;
impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {

        if s1.len() > s2.len() {return false};


        let mut map: HashMap<char,i32> = HashMap::with_capacity(26);
        for chars in s1.chars(){
            let val= map.entry(chars).or_insert(0);
            *val +=1 ;
        }

        let mut left = 0;
        let mut right = s1.len() -1;
        let s2_bytes = s2.as_bytes();
        while right < s2.len(){

            let mut maptemp = map.clone();
            let mut flag = true;

            for i in left..=right{
                if let Some(val)= maptemp.get_mut(&(s2_bytes[i as usize] as char)){
                    *val -=1;
                }  
            }

            for (key, &val) in maptemp.iter(){
                if val!=0 {
                    flag = false;
                    continue;}
            }
            if flag {return true;}


            left+=1;
            right+=1;
        }


        false
    }   
}
