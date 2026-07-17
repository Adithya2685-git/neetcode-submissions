use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct KthLargest {
    q: BinaryHeap<i32>,
    k: i32
}

impl KthLargest {
    pub fn new(k: i32, nums: Vec<i32>) -> Self {

        let mut q = BinaryHeap::new();
        for elements in nums{
            q.push(elements);
        }
        KthLargest{q: q, k: k}
    }

    pub fn add(&mut self, val: i32) -> i32 {
        self.q.push(val);
        let k = self.k;
        let mut result = 0;

        let mut q = Vec::new();

        for i in 0..k{
            if let Some(element) = self.q.pop(){
                result= element;
                q.push(element);
            }
        }

        while let Some(element) = q.pop(){
            self.q.push(element);
        }
    result
    }
}
