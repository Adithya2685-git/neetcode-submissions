impl Solution {

    pub fn rob(nums: Vec<i32>) -> i32 {
        
        let mut prev =nums[0];
        let mut prev2=0 ;
        for i in 1..nums.len(){

            let mut take = nums[i];
            if i>=2{take +=prev2;}
            let mut skip= prev;

            let curri = take.max(skip);
            prev2 = prev;
            prev = curri;
        }
        prev
    }
}
