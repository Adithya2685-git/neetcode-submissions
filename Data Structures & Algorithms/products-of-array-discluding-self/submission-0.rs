impl Solution {
    pub fn product_except_self(mut nums: Vec<i32>) -> Vec<i32> {
        let mut zindex = -1;
        let mut product = 1;
        for (i,&element) in nums.iter().enumerate(){
            if element== 0{
                if zindex==-1{
                    zindex = i as i32;
                }else{
                    return vec![0;nums.len()];
                }
            }else{
                product*=element;
            }
        }

        if zindex==-1{
            for i in 0..nums.len(){
                nums[i] = product/ nums[i];
            }
            nums
        }else{
            let mut v =vec![0; nums.len()];
            v[zindex as usize] = product;
            v
        }
    }
}
