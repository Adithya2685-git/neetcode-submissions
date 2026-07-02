use std::ops::BitXor;
impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut x: i32 = 0;

        for (i,&element) in nums.iter().enumerate(){
            x = x.bitxor(i as i32);
            x= x.bitxor(element);
        }

        x = x.bitxor( nums.len() as i32); 
        x
    }
}
