impl Solution {

    pub fn tree() {

    }
    pub fn can_partition(mut nums: Vec<i32>) -> bool {
        
        let total: i32 = nums.iter().sum() ;
        if total%2 != 0{
            return false;
        }

        let target = total/2;

        let mut dp = vec![false; (target+1) as usize];
        dp[0]= true;

        for &elements in nums.iter(){
            
            let ele = elements as usize;

            for i in (ele..=target as usize).rev(){
                if dp[i - ele] && elements <= target{
                    dp[i] =true;
                }
            }
        }


    *dp.last().unwrap()
    }
}
