impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {

        let gsum: i32 = gas.iter().sum();
        let csum: i32 = cost.iter().sum(); 
    
        if gsum < csum{
            return -1
        }


        let mut index = 0;

        let mut total =0;
        let n = gas.len();
        for i in 0..n{

            total += gas[i] - cost[i];
            if total<0{
                index= (i+1)%n;
                total = 0;
            }

        }

        index as i32
    }
}
