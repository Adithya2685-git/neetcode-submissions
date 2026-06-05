impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        
        let mut a= Vec::with_capacity(2* nums.len());

        a.extend(&nums); 
        a.extend(&nums);

        a

    }
}
