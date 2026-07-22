use std::collections::{HashSet, VecDeque};
impl Solution {
    
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {

        let len = word_list[0].len();
        let mut q:VecDeque<Vec<u8>> = VecDeque::new();
        q.push_back(begin_word.as_bytes().to_vec());
        let mut set: HashSet<Vec<u8>> = HashSet::new();
        for w in word_list.iter(){
            set.insert(w.as_bytes().to_vec());
        }

        let mut depth = 1;
        let endword = end_word.as_bytes();
        while !q.is_empty(){
            let levelsize =q.len();

            for _ in 0..levelsize {
                let w_bytes = q.pop_front().unwrap();
                let mut trans= w_bytes.clone();

                for i in 0..len{ 
                    let original = trans[i];
                    for c in b'a'..=b'z' {
                        trans[i] = c;

                        if trans!= w_bytes && set.contains(&trans){
                            q.push_back(trans.clone());
                            set.remove(&trans);
                            if endword  == trans{
                                return depth+1;
                            }
                        } 

                    }
                    trans[i]= original;
                }
            }

            depth+=1;
        }

        0
    }
}
