use std::collections::VecDeque;
struct MyQueue {
    q: VecDeque<i32>
}

impl MyQueue {
    pub fn new() -> Self {
        let que = VecDeque::new();
        MyQueue { q: que}
    }

    pub fn push(&mut self, x: i32) {
        self.q.push_back(x);
    }

    pub fn pop(&mut self) -> i32 {
        if let Some(x) = self.q.pop_front() {return x;}
        -1
    }

    pub fn peek(&self) -> i32 {
        if let Some(x) = self.q.front() {return *x;}
        -1
    }

    pub fn empty(&self) -> bool {
        self.q.is_empty()
    }
}

// Your MyQueue object will be instantiated and called as such:
// let obj = MyQueue::new();
// obj.push(x);
// let ret_2: i32 = obj.pop();
// let ret_3: i32 = obj.peek();
// let ret_4: bool = obj.empty();
