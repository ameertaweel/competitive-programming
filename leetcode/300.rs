// LeetCode/300 - Longest Increasing Subsequence

use std::cmp::max;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut seq = vec![nums[0]];
        let mut longest = 1;
        for &n in &nums[1..] {
            if n > seq[seq.len() - 1] {
                seq.push(n);
                longest += 1;
                continue;
            }
            let i = match seq.binary_search(&n) {
                Ok(i) => i,
                Err(i) => i,
            };
            seq[i] = n;
        }
        return longest;
    }

    pub fn length_of_lis_alt_1(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp = vec![1; n];
        let mut longest = 1;

        for i in 0..n {
            for j in 0..i {
                if nums[i] > nums[j] {
                    dp[i] = max(dp[i], dp[j] + 1);
                    longest = max(longest, dp[i]);
                }
            }
        }

        return longest;
    }

    pub fn length_of_lis_alt_2(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp = vec![1; n];
        let mut longest = 1;

        for i in (0..n).rev() {
            for j in (i + 1)..n {
                if nums[i] < nums[j] {
                    dp[i] = max(dp[i], dp[j] + 1);
                    longest = max(longest, dp[i]);
                }
            }
        }

        return longest;
    }
}
