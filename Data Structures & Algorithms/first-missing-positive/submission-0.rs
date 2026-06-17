use std::collections:: HashSet;
impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut mini = 0;
        let mut set = HashSet::new();

        let mut flag = false;
        for element in nums.iter(){
            if *element > 0{
                set.insert(*element);
            }
        }

        while !flag{
            mini+=1;
            if !set.contains(&mini){
                flag = true;
            }
        } 

        mini

    }
}
