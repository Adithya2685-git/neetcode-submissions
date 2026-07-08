impl Solution {
    pub fn insert(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let a = new_interval[0];
        let mut ind = 0;
        
        for i in 0..intervals.len() {
            if intervals[i][0] <= a {
                ind = i + 1;
            }
        }
        intervals.insert(ind, new_interval);

        let mut res = Vec::new();
        let mut a = intervals[0][0];
        let mut b = intervals[0][1];
        for i in 0..intervals.len(){
            
            if b >= intervals[i][0]{
                b = b.max(intervals[i][1]);
            }else{
                res.push(vec![a,b]);
                a = intervals[i][0];
                b = intervals[i][1];
            }
        }
        res.push(vec![a,b]);
        res
    }
}
