class Solution {
public:
    int recurse(vector<int> &nums, int i,int n,vector<int> &dp){
        if(i==0){
            return nums[i];
        }
        if (dp[i]!=-1e7){
            return dp[i];
        }
        int choose= nums[i] + recurse(nums, i-1, n,dp);
        int reset = nums[i];
        return dp[i]=max(reset, choose);
    }

    int maxSubArray(vector<int>& nums) {

        int sum = INT_MIN;
        int curr= 0;
        for(int i=0;i<nums.size();i++){
            if(curr<0){
                curr=nums[i];
            }else{
                curr+=nums[i];
            }
            sum = max(sum, curr);
        }
        return sum;
    }
};
