use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut h = HashMap::new();
        for c in s.chars() {
            *h.entry(c).or_insert(0) += 1;
        }
        for c in t.chars() {
            *h.entry(c).or_insert(0) -= 1;
        }
        for &v in h.values() {
            if v != 0 {
                return false;
            }
        }
        return true;
    }

    // Alternative Solution
    // Slower than the main solution
    pub fn is_anagram_alt(s: String, t: String) -> bool {
        let mut sv: Vec<char> = s.chars().collect();
        sv.sort();
        let mut st: Vec<char> = t.chars().collect();
        st.sort();
        return sv == st;
    }
}
