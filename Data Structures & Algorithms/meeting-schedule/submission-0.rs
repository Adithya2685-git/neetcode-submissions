/**
 * Definition of Interval:
 * #[derive(Debug, Clone)]
 * pub struct Interval {
 *     pub start: i32,
 *     pub end: i32,
 * }
 *
 * impl Interval {
 *     pub fn new(start: i32, end: i32) -> Self {
 *         Interval { start, end }
 *     }
 * }
 */

use std::collections::VecDeque;

impl Solution {
    pub fn can_attend_meetings(mut intervals: Vec<Interval>) -> bool {
        intervals.sort_by(|a,b| a.start.cmp(&b.start));
        let mut q:Vec<Interval> = Vec::new();    
        
        for times in intervals{  
            if let Some(latest) = q.last(){
                if latest.end > times.start{
                    return false;
                }
            }
            q.push(times);
        }
    
        true
    }
}
