impl Solution {
    pub fn recursion(prices: &Vec<i32> , i: usize, holding:bool, dp : &mut Vec<Vec<i32>> )-> i32{
        if i>=prices.len(){
            if holding{
                return -1;
            }else{
                return 0;
            }
        }

        let h = holding as usize;
        if dp[i][h]!= -1{
            return dp[i][h];
        }


        let result;
        if holding{
            let sell= prices[i] + Self::recursion(prices,i+1, false, dp);

            let skip= Self::recursion(prices, i+1, holding,dp);
            result =sell.max(skip);
        }else{

            let buy= -prices[i] + Self::recursion(prices, i+1, true,dp);
            let skip  = Self::recursion(prices, i+1, holding, dp);
            result = buy.max(skip);
        }

        dp[i][h]= result;
        result
    }

    pub fn max_profit(prices: Vec<i32>) -> i32 {

        let mut dp= vec![vec![-1;2]; prices.len()];
        Self::recursion(&prices, 0, false, &mut dp)
    }
}
