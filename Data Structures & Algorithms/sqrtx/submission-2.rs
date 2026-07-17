impl Solution {
    pub fn bs(x: i32, left: i32, right: i32)-> i32{
        if left>= right{
            return left;
        }
        let mut mid = left + (right-left)/2;
        if mid as i64 *mid as i64 >x as i64{
            Self::bs(x,left, mid)

        }else{
            Self::bs(x,mid+1, right)
        }


    }


    pub fn my_sqrt(x: i32) -> i32 {
        if x < 2{
            x
        }else{
            Self::bs(x,0,x) -1

        }
    }
}
