impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {

        let mut min = prices[0];
        let mut max_profit =0 ;
        for &element in prices.iter(){

            min = min.min(element);

            let profit = element - min;

            max_profit = max_profit.max(profit);

        }
        max_profit
    }
}
