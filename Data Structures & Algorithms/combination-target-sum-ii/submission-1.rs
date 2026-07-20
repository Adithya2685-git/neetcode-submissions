impl Solution {
    pub fn recursion(candidates: &Vec<i32>, i: usize, v: &mut Vec<i32>, set: &mut Vec<Vec<i32>>,target: i32 ){
        if target ==0{
            set.push(v.clone());
            return;
        }

        for index in i..candidates.len(){
            if index >i && candidates[index]== candidates[index-1]{continue;}
            
            if candidates[index] > target{
                break;
            }

            v.push(candidates[index]);
            Self::recursion(candidates,index+1, v, set, target- candidates[index]);
            v.pop();
        }
    }

    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort();
        let mut v = Vec::new();
        let mut set= Vec::new();
        Self::recursion(&candidates,0,&mut v, &mut set, target,);

        set
    }
}
