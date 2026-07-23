
class Solution:
    def recursion( self, nums:List[int], i: int,n:int,dp)-> int:
        if i<0:
            return 0

        if dp[i]!= -1:
            return dp[i]
        
        rob= nums[i]
        if i>=1:
            rob += self.recursion(nums, i-2,n,dp)

        skip= self.recursion(nums, i-1,n,dp )

        dp[i]= max(rob, skip)
        return dp[i]

    def rob(self, nums: List[int]) -> int:
        n = len( nums)
        dp = [-1] * n
        return self.recursion(nums,len(nums)-1,n,dp)