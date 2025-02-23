// LeetCode/139 - Word Break

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let n = s.len();

        let mut dp = vec![false; n + 1];
        dp[0] = true;

        for i in 0..n {
            for w in &word_dict {
                if i + 1 >= w.len() && dp[(i + 1) - w.len()] && &s[(i - w.len() + 1)..=i] == w {
                    dp[i + 1] = true;
                    break;
                }
            }
        }

        return dp[n];
    }
}
