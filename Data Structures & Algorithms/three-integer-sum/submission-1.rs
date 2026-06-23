use std::collections::HashSet;
impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {

        nums.sort();
        let mut set:HashSet<(i32,i32,i32)> = HashSet::new();
        for (i,&element) in nums.iter().enumerate(){
            let target = -element;

            let mut left = i+1 ;
            let mut right = nums.len() -1;

            while left < right{
                let sum =nums[left]+ nums[right];
                if sum < target{
                    left +=1;
                }else if sum > target{
                    right -=1;
                }else{
                    if i != left && i!=right{
                        set.insert((nums[left], nums[right],element));
                        left+=1;
                        right-=1;    
                    }

                }

            }

        }

        set.into_iter().collect::<Vec<(i32,i32,i32)>>().into_iter().map(|(a, b, c)| vec![a, b, c]).collect()
    }
}
