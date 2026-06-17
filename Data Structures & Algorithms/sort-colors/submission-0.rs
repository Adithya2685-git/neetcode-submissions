use std::collections::HashMap;
impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut map: HashMap<i32,i32> = HashMap::new();

        for elements in nums.iter(){
            let val = map.entry(*elements).or_insert(0);
            *val+=1; 
        }

        let mut iter = nums.iter_mut();
        
        for color in 0..3{
            
            if let Some(&count) = map.get(&color){
                
                for _ in 0..count {

                    if let Some(entry) = iter.next(){
                    *entry= color;
                    }
                }
            }
        

        }
    }
}
