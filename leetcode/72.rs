// LeetCode/72 - Edit Distance

use std::cmp::min;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        if word1.len() == 0 {
            return word2.len() as i32;
        }
        if word2.len() == 0 {
            return word1.len() as i32;
        }
        let (w1, w2) = if word1.len() >= word2.len() {
            (word1.as_bytes(), word2.as_bytes())
        } else {
            (word2.as_bytes(), word1.as_bytes())
        };
        let (n1, n2) = (w1.len(), w2.len());
        let mut dp = vec![vec![0; n2 + 1]; n1 + 1];
        for i in 0..=n1 {
            dp[i][0] = i as i32;
        }
        for j in 0..=n2 {
            dp[0][j] = j as i32;
        }
        for i in 1..=n1 {
            for j in 1..=n2 {
                let m = if w1[i - 1] == w2[j - 1] {
                    dp[i - 1][j - 1]
                } else {
                    dp[i - 1][j - 1] + 1
                };
                let d = dp[i - 1][j] + 1;
                let r = dp[i][j - 1] + 1;

                dp[i][j] = min(m, min(d, r));
            }
        }
        return dp[n1][n2];
    }
}
