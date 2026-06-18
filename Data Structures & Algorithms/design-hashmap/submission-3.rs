struct MyHashMap {
    key:  Vec<i32>,
    val: Vec<i32>
}

impl MyHashMap {
    pub fn new() -> Self {
        let key = Vec::new();
        let val = Vec::new();

        MyHashMap{key:key, val:val}
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if let Some(index) = self.key.iter().position(|x| *x == key) {
            self.val[index] = value;
        }else{
            self.key.push(key);
            self.val.push(value);
        }
    }

    pub fn get(&self, key: i32) -> i32 {
        let mut result = -1; 
        if let Some(index) = self.key.iter().position(|x| *x == key) {
            
            result = self.val[index];
        }
        result
    }

    pub fn remove(&mut self, key: i32) {
        let mut result = -1; 
        if let Some(index) = self.key.iter().position(|x| *x == key) {
        
            self.key.remove(index);
            self.val.remove(index);
        }
    }
}
