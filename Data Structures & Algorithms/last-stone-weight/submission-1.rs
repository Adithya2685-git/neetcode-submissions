use std::collections::BinaryHeap;
impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {

        let mut heap = BinaryHeap::new();

        for element in stones{
            heap.push(element);
        }

        while heap.len()>1{
            let x = heap.pop().unwrap();
            let y = heap.pop().unwrap();

            if x==y{
                continue;
            }else{
                heap.push(x-y);
            }

        }

        if let Some(weight) = heap.pop(){
            weight
        }else{
            0
        }
    }
}
