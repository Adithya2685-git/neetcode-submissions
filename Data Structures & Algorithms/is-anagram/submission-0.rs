use std::collections::HashMap;
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {

        let mut map : HashMap<char, i32> = HashMap::new();

        for element in s.chars(){
            
            let count = map.entry(element).or_insert(0);
            *count += 1;

        }

        for element in t.chars(){ 

            let count = map.entry(element).or_insert(-1);
            *count -=1;
        }

        for (key,val) in map.iter(){
            
            if *val != 0{
                return false;
            }
        }
        true
        
    }
}
