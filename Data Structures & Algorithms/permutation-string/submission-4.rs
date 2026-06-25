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
        
        let mut maptemp = map.clone();

        while right < s2.len(){
            maptemp.clone_from(&map);

            let mut flag = true;

            for i in left..=right{
                if let Some(val)= maptemp.get_mut(&(s2_bytes[i as usize] as char)){
                    *val -=1;
                }  
            }

            for (key, &val) in maptemp.iter(){
                if val!=0 {
                    flag = false;
                    break;}
            }
            if flag {return true;}


            if let Some(val)= maptemp.get_mut(&(s2_bytes[left as usize] as char)){
                *val -=1;
            } 

            left+=1;
            right+=1;
            
            if right < s2.len() && let Some(val)= maptemp.get_mut(&(s2_bytes[right as usize] as char)) {
                *val +=1;
            }

        }


        false
    }   
}
