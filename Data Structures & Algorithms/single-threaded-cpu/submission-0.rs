use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn get_order(tasks: Vec<Vec<i32>>) -> Vec<i32> {
        
        // 1. Map to a tuple: (original_index, enqueue_time, processing_time)
        let mut track: Vec<(usize, i32, i32)> = tasks
            .into_iter()
            .enumerate()
            .map(|(i, t)| (i, t[0], t[1]))
            .collect();
        
        // 2. Sort by enqueue_time
        track.sort_unstable_by_key(|task| task.1);

        let mut result = Vec::new();
        let mut pq = BinaryHeap::new();
        
        let mut time = 0i64; // i64 prevents integer overflow!
        let mut i = 0;       // Our pointer for the 'track' array
        
        // 3. Loop until ALL tasks are processed (both the array and the heap are empty)
        while i < track.len() || !pq.is_empty() {
            
            // If the CPU is idle and the heap is empty, fast-forward time to the next task!
            if pq.is_empty() && time < track[i].1 as i64 {
                time = track[i].1 as i64;
            }
            
            // 4. Enqueue ALL tasks that have arrived up to the current 'time'
            while i < track.len() && track[i].1 as i64 <= time {
                
                // We only push (processing_time, original_index).
                // The heap will naturally prioritize processing time, and use index as a tie-breaker.
                pq.push(Reverse((track[i].2, track[i].0)));
                i += 1;
            }
            
            // 5. Pop the shortest task, add it to our result, and advance time
            if let Some(Reverse((dur, idx))) = pq.pop() {
                result.push(idx as i32);
                time += dur as i64;
            }
        }
        
        result
    }
}