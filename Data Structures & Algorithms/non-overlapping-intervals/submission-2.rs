impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        
        intervals.sort_by(|a,b| {
            a[1].cmp(&b[1]).then(a[0].cmp(&b[0]))
        });

        let mut stack = Vec::new();
        for pairs in intervals.iter(){
            if stack.is_empty(){stack.push(pairs); continue;}

            if let Some(last) = stack.last(){
                let a1 = pairs[0];

                let b0 = last[1];

                if b0<= a1{
                    stack.push(pairs);
                }
            }
        }

        (intervals.len() - stack.len()) as i32
    }
}
