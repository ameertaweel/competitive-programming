impl Solution {
    pub fn trap(heights: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut j = heights.len() - 1;
        let mut max_h_left = 0;
        let mut max_h_rght = 0;

        let mut water = 0;

        while i <= j {
            let hi = heights[i];
            let hj = heights[j];

            if hi >= max_h_left {
                max_h_left = hi;
                i += 1;
                continue;
            }
            if hj >= max_h_rght {
                max_h_rght = hj;
                j -= 1;
                continue;
            }

            if max_h_left < max_h_rght {
                water += max_h_left - hi;
                i += 1;
            } else {
                water += max_h_rght - hj;
                j -= 1;
            }
        }

        return water;
    }
}
