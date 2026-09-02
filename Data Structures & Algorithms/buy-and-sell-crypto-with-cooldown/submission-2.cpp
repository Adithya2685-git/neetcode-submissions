class Solution {
public:
    int recurse(vector<int> &prices, int i, bool holding, vector<vector<int>> &dp){
        if(i>=prices.size()){
            return 0;
        }
        if(dp[i][holding]!= -1){
            return dp[i][holding];
        }
        int hold = recurse(prices, i+1, holding,dp);
        if(!holding){
            int buy = -prices[i]+ recurse(prices, i+1, true,dp);
            return dp[i][holding]=max(hold, buy);
        }else{
            int sell = prices[i]+ recurse(prices,i+2, false,dp);
            return dp[i][holding]=max(hold, sell );
        }
    }
    int maxProfit(vector<int>& prices) {
        vector<vector<int>> dp(prices.size(), vector<int>(2,-1));
        return recurse(prices,0, false, dp);
    }
};
