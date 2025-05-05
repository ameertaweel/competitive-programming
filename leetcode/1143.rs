// LeetCode/1143 - Longest Common Subsequence

use std::cmp::max;

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let (text1, text2) = if text1.len() < text2.len() {
            (text1, text2)
        } else {
            (text2, text1)
        };
        let n1 = text1.len();
        let n2 = text2.len();
        let c1: Vec<_> = text1.chars().collect();
        let c2: Vec<_> = text2.chars().collect();
        let mut dp = vec![vec![0; n1 + 1]; 2];

        for i2 in 1..=n2 {
            for i1 in 1..=n1 {
                let diag = if c1[i1 - 1] == c2[i2 - 1] {
                    dp[(i2 + 1) % 2][i1 - 1] + 1
                } else {
                    dp[(i2 + 1) % 2][i1 - 1]
                };
                dp[i2 % 2][i1] = max(diag, max(dp[(i2 + 1) % 2][i1], dp[i2 % 2][i1 - 1]));
            }
        }
        return dp[n2 % 2][n1];
    }
}
