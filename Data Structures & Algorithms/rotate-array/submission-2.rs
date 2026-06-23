impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let len = nums.len() as i32; 
        nums.rotate_right((k% len) as usize)
    }
}
