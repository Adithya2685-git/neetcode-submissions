impl Solution {
    pub fn merge(a: Vec<i32>,b:Vec<i32>)-> Vec<i32>{

        let mut itera = a.iter();
        let mut iterb = b.iter();
        let mut merged = Vec::new();
        
        let mut curra = itera.next();
        let mut currb = iterb.next();

        while let (Some(&vala), Some(&valb)) = (curra,currb) {
            if vala < valb {
                merged.push(vala);
                curra = itera.next();
            }else{
                merged.push(valb);
                currb = iterb.next();                
            }

        } 

        while let Some(&elementa) = curra{
            merged.push(elementa);
            curra = itera.next()
        }
        while let Some(&elementb) = currb {
            merged.push(elementb);
            currb = iterb.next();
        }
        merged
    }

    pub fn mergesort(a: &Vec<i32>, left: usize , right: usize)-> Vec<i32>{

        if right- left <= 1 {return a[left..right].to_vec()}

        let mid = left + (right- left)/2;

        let arr = Self::mergesort(a,left,mid);
        let brr = Self::mergesort(a,mid, right);

        Self::merge(arr, brr)
    }

    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        
        let result = Self::mergesort(&nums,0,nums.len());
        result
    }
}
