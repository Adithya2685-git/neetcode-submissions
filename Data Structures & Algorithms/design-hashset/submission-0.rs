use std::collections::BTreeSet;
struct MyHashSet {
    arr:BTreeSet<i32>
}

impl MyHashSet {
    pub fn new() -> Self {

        let arr = BTreeSet::new();
        
        MyHashSet{arr:arr}
        
    }

    pub fn add(&mut self, key: i32) {
        self.arr.insert(key);
    }

    pub fn remove(&mut self, key: i32) {
        self.arr.remove(&key);

    }

    pub fn contains(&self, key: i32) -> bool {
        self.arr.contains(&key)
    }
}
