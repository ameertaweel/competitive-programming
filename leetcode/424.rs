impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        if s.len() == 0 {
            return 0;
        }

        let k = k as usize;

        // Occurrences
        let mut o = vec![0; 26];
        let vals: Vec<_> = s.chars().map(|c| (c as usize) - ('A' as usize)).collect();
        let mut beg = 0; // Inclusive
        let mut end = 0; // Exclusive

        let mut max_len = 0;

        while end < s.len() || beg < s.len() - 1 {
            if end < s.len() {
                o[vals[end]] += 1;
                end += 1;
                while (end - beg) - o[vals[beg]] > k {
                    o[vals[beg]] -= 1;
                    beg += 1;
                }
            } else {
                o[vals[beg]] -= 1;
                beg += 1;
            }

            let replacements = (end - beg) - o[vals[beg]];
            let len = (end - beg) + (k - replacements);
            if len >= s.len() {
                return s.len() as i32;
            }
            if len > max_len {
                max_len = len;
            }
        }

        return max_len as i32;
    }
}
