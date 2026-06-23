impl Solution {
    pub fn max_area(heights: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = heights.len() -1;

        let mut max_area = i32::MIN;

        while left < right {

            let area = (right as i32 - left as i32) *(heights[left].min(heights[right]));
            max_area = area.max(max_area);

            if heights[left] < heights[right]{
                left+=1;
            }else{
                right -=1;
            }
        }
        max_area
    }
}
