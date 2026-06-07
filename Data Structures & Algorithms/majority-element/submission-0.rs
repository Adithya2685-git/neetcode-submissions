impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {

        let (_, candid) = nums.into_iter().fold((0,0),|(mut count, candidate), element|{
            if count == 0{ 
                return (1, element)
            }

            if element == candidate{
                count+=1;
            }else{
                count -=1;
            }

            (count, candidate)
        });
            
        candid
    }
}
