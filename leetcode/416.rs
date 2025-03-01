// LeetCode/416 - Partition Equal Subset Sum

use std::collections::HashMap;

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let sum = nums.iter().sum::<i32>();
        if sum % 2 == 1 {
            return false;
        }
        let target = sum / 2;
        // 0/1 Knapsack Problem
        let mut dp = vec![false; (target + 1) as usize];
        dp[0] = true;
        for i in 0..nums.len() {
            for j in (1..=target).rev() {
                if j - nums[i] >= 0 {
                    dp[j as usize] = dp[j as usize] || dp[(j - nums[i]) as usize];
                }
            }
        }
        return dp[target as usize];
    }
}
