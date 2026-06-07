impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        
        let  cloned = nums.clone();

        let mut iter = nums.iter_mut();
        let mut ans = 0;
        for element in cloned.iter(){
            if *element == val{
                ans +=1;
                continue;
            }
            *iter.next().unwrap() = *element;
         
        }
        nums.len() as i32 - ans 
    }
    
}
