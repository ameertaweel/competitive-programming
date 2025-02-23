// LeetCode/5 - Longest Palindromic Substring

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let n = s.len();
        let chars: Vec<_> = s.chars().collect();

        let mut dp_even = vec![true; n];
        let mut dp_odd = vec![true; n];
        let mut longest = (0, 0);

        let mut found_even = true;
        let mut found_odd = true;
        for w in 2..=n {
            let dp = if w % 2 == 0 {
                &mut dp_even
            } else {
                &mut dp_odd
            };
            let found = if w % 2 == 0 {
                &mut found_even
            } else {
                &mut found_odd
            };

            *found = false;

            for beg in 0..=(n - w) {
                let end = beg + w - 1;
                let is_palindromic = dp[beg + 1] && chars[beg] == chars[end];
                dp[beg] = is_palindromic;
                if is_palindromic && w > (longest.1 - longest.0 + 1) {
                    *found = true;
                    longest = (beg, end);
                }
            }

            if !found_even && !found_odd {
                break;
            }
        }

        return s[longest.0..=longest.1].to_string();
    }
}
