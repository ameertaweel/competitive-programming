// LeetCode/76 - Minimum Window Substring

use std::collections::HashMap;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let s: Vec<_> = s.chars().collect();
        let t: Vec<_> = t.chars().collect();

        // Multiplicities
        let mut m = HashMap::new();
        let mut count = 0;

        for &c in &t {
            if !m.contains_key(&c) {
                m.insert(c, 0);
            }
            *m.get_mut(&c).unwrap() += 1;
            count += 1;
        }

        let (mut ans_a, mut ans_b) = (0, 0);

        let (mut a, mut b) = (0, 0);
        while a < s.len() {
            while count > 0 && b < s.len() {
                b += 1;
                if !m.contains_key(&s[b - 1]) {
                    continue;
                }
                if m[&s[b - 1]] > 0 {
                    count -= 1;
                }
                *m.get_mut(&s[b - 1]).unwrap() -= 1;
            }
            if count > 0 {
                break;
            }
            while a < b {
                if !m.contains_key(&s[a]) {
                    a += 1;
                    continue;
                }
                if m[&s[a]] == 0 {
                    break;
                }
                *m.get_mut(&s[a]).unwrap() += 1;
                a += 1;
            }
            if (ans_a == 0 && ans_b == 0) || (b - a < ans_b - ans_a) {
                (ans_a, ans_b) = (a, b);
            }
            *m.get_mut(&s[a]).unwrap() += 1;
            a += 1;
            count += 1;
        }

        return s[ans_a..ans_b].iter().collect();
    }
}
