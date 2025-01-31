impl Solution {
    pub fn max_area(heights: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut j = heights.len() - 1;

        let mut max_capacity = 0;

        while i < j {
            let width = (j - i) as i32;
            let height = std::cmp::min(heights[i], heights[j]);
            let capacity = width * height;
            if capacity > max_capacity {
                max_capacity = capacity;
            }

            if heights[i] == heights[j] {
                i += 1;
                j -= 1;
            } else if heights[i] > heights[j] {
                j -= 1;
            } else {
                i += 1;
            }
        }

        return max_capacity;
    }
}
