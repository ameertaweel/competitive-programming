// LeetCode/560 - Subarray Sum Equals K

use std::collections::HashMap;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut sum = 0;
        let mut partial_sums_counts = HashMap::new();
        partial_sums_counts.insert(0, 1);
        let mut ans = 0;
        for n in nums {
            sum += n;
            let diff = sum - k;
            if partial_sums_counts.contains_key(&diff) {
                ans += partial_sums_counts[&diff];
            }
            *partial_sums_counts.entry(sum).or_insert(0) += 1;
        }
        return ans;
    }
}
