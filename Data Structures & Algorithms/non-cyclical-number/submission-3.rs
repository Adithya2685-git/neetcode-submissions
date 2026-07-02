impl Solution {
    pub fn is_happy(mut n: i32) -> bool {
    

        while n >1{
            if n<10{
                if n== 3 || n == 9 || n==2 || n==4 || n == 5|| n== 6 || n==8{
                    return false;
                }else{
                    return true;
                }
            }

            let mut sum = 0;

            let mut d = 0; 
            let mut temp = n;

            while temp> 0{
                d = temp%10; 
                sum += d*d; 

                temp/= 10;
            }

            n = sum;
        }
        
        true
    }
}
