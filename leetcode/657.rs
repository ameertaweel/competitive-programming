// LeetCode/657 - Palindromic Substrings

impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let n = s.len();
        let chars: Vec<_> = s.chars().collect();

        let mut dp_even = vec![true; n];
        let mut dp_odd = vec![true; n];

        let mut count = n;

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
                if is_palindromic {
                    *found = true;
                    count += 1;
                }
            }

            if !found_even && !found_odd {
                break;
            }
        }

        return count as i32;
    }
}
