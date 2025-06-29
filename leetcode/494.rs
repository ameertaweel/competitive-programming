// LeetCode/494 - Target Sum

use std::collections::HashMap;

impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let mut rem = nums.iter().sum::<i32>();
        let mut dp = HashMap::new();
        dp.insert(0, 1);

        for n in &nums {
            rem -= n;
            let mut dp_next = HashMap::new();
            for (k, v) in &dp {
                if ((k + n) - target).abs() <= rem {
                    *dp_next.entry(k + n).or_insert(0) += v;
                }
                if ((k - n) - target).abs() <= rem {
                    *dp_next.entry(k - n).or_insert(0) += v;
                }
            }
            dp = dp_next;
        }

        return *dp.get(&target).unwrap_or(&0);
    }
}
