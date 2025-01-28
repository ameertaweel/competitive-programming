use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut h = HashMap::new();
        for s in strs {
            // Uses the fact that `s` contains only English lowercase letters
            let mut counts = [0 as u8; 26];
            for c in s.chars() {
                let index = c as u8 - 'a' as u8;
                counts[index as usize] += 1;
            }

            if !h.contains_key(&counts) {
                h.insert(counts, vec![s]);
            } else {
                h.get_mut(&counts).unwrap().push(s);
            }
        }

        return h.values().cloned().collect();
    }

    // Alternative Solution
    // Slower than the main solution
    pub fn group_anagrams_alt(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut h = HashMap::new();
        for s in strs {
            let mut a: Vec<char> = s.chars().collect();
            a.sort();

            if !h.contains_key(&a) {
                h.insert(a, vec![s]);
            } else {
                h.get_mut(&a).unwrap().push(s);
            }
        }

        return h.values().cloned().collect();
    }
}
