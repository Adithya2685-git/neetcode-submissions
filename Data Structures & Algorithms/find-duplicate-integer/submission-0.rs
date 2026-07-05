use std::collections::HashSet;
impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut set = HashSet::new();

        for element in nums.iter(){
            if set.contains(element){
                return *element;
            }else{
                set.insert(*element);
            }
        }
        -1
    }
}
