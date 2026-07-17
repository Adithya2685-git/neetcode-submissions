use std::collections::HashMap;
impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {

        let mut map = HashMap::new();
        for (i, chars) in order.chars().enumerate(){
            map.insert(chars, i);
        }

        let mut v = Vec::new();
        let mut prev = Vec::new();

        for word in words{
            for c in word.chars(){
                if let Some(i) = map.get(&c){
                    v.push(i);
                }
            }
            if v < prev{
                return false
            }
            prev = v.clone();
            v.clear();
        }
        true
    }
}