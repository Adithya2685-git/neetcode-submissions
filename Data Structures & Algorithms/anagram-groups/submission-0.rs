use std::collections::HashMap;
impl Solution {


    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {

        let mut map: HashMap<[u8; 26], Vec<String>> = HashMap::new();


        for string in strs.into_iter(){
            
            let mut counter = [0; 26];

            for chars in string.as_bytes().iter(){
                
                let element = (*chars - b'a') as usize;

                counter[element] += 1;
            }

            map.entry(counter).or_default().push(string);


        }

        map.into_values().collect()

    }
}
