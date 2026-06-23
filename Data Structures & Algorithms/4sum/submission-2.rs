use std::collections::HashSet;
impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        nums.sort();
        let mut set:HashSet<(i32,i32,i32,i32)> = HashSet::new();

        for (i,&element) in nums.iter().enumerate(){

            for j in i+1..nums.len(){

                let preztarget = (target as i64 - element as i64  - nums[j] as i64 );

                let mut left = j+1 ;
                let mut right = nums.len() -1;

                while left < right{
                    let sum =(nums[left] as i64 + nums[right] as i64);
                        if sum < preztarget{
                            left +=1;
                        }else if sum > preztarget{
                            right -=1;
                        }else{
                            if i != left && i!=right{
                                set.insert((element,nums[j],nums[left], nums[right]));
                                left+=1;
                                right-=1;    
                            }
                        }
                    }
                }

            }

            set.into_iter().map(|(a, b, c, d)| vec![a, b, c, d]).collect()        
        }
    }

