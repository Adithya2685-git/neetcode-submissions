
impl Solution {

    pub fn sums(piles: &Vec<i32>, k: i64)-> i64{
        let mut x  = 0;

        for &element in piles.iter(){
            x += (element as f64/ k as f64).ceil() as i64;
        }
        x
    }

    pub fn binarysearch(piles: &Vec<i32> ,left: i64 , right: i64,h: i64)-> i64{
        if left>= right{
            return left;
        }

        let mid = left + (right-left)/2;

        let result = Self::sums(piles,mid);
        

        if result> h{
            Self::binarysearch(piles,mid+1, right, h)
        }else{
            Self::binarysearch(piles,left, mid, h)
        }



    }

    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        if let Some(max) = piles.iter().max(){
            return Self::binarysearch(&piles,1, *max as i64, h as i64) as i32;            
        }
        -1
    }
}
