// LeetCode/97 - Interleaving String

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let (s1, s2, s3) = (s1.as_bytes(), s2.as_bytes(), s3.as_bytes());

        let n1 = s1.len();
        let n2 = s2.len();
        let n3 = s3.len();

        if n1 + n2 != n3 {
            return false;
        }

        let mut dp = vec![false; n2 + 1];
        dp[0] = true;

        for row in 0..=n1 {
            for col in 0..=n2 {
                dp[col] = (row == 0 && col == 0)
                    || (row > 0 && dp[col] && s1[row - 1] == s3[row + col - 1])
                    || (col > 0 && dp[col - 1] && s2[col - 1] == s3[row + col - 1]);
            }
        }

        return dp[n2];
    }
}
