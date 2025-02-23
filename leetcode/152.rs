// LeetCode/152 - Maximum Product Subarray

use std::cmp::{max, min};

impl Solution {
    // Solution Using Kadane's Algorithm
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut curr_max = 1;
        let mut curr_min = 1;
        let mut result = nums[0];

        for n in nums {
            let next_max = max(max(curr_max * n, curr_min * n), n);
            let next_min = min(min(curr_max * n, curr_min * n), n);
            curr_max = next_max;
            curr_min = next_min;
            result = max(result, curr_max);
        }

        return result;
    }
}
