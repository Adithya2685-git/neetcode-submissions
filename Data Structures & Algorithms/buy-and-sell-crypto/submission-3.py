class Solution:
    def recursion(self, holding,sold: int,i: int, prices: list[int])-> int:
        if i>=len(prices):
            if holding==1:
                return -1
            else:
                return 0

        if sold ==1:
            return 0
            
        if holding ==1 :

            sell = prices[i] + self.recursion(0, 1,i+1, prices)
            skip = self.recursion(holding,sold, i+1, prices)
            return max(sell,skip)

        else:
            buy= -prices[i] +self.recursion(1,sold,i+1 , prices)
            skip = self.recursion(holding,sold, i+1, prices)
            return max(buy, skip)


    def maxProfit(self, prices: List[int]) -> int:

        return self.recursion(0 ,0,0, prices)
