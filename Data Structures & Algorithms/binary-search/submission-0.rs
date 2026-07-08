impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if let Ok(ans) = nums.binary_search(&target){
            ans as i32
        }else{
            -1
        }
    }
}
