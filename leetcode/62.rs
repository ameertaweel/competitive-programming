// LeetCode/62 - Unique Paths

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let (m, n) = (m as usize, n as usize);
        let mut dp = vec![vec![1; n]; m];
        for i in 1..m {
            for j in 1..n {
                let top = dp[i - 1][j];
                let left = dp[i][j - 1];
                dp[i][j] = top + left;
            }
        }
        return dp[m - 1][n - 1] as i32;
    }
}
