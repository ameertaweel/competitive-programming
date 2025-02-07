impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let s1: Vec<_> = s1.chars().map(|c| (c as usize) - ('a' as usize)).collect();
        let s2: Vec<_> = s2.chars().map(|c| (c as usize) - ('a' as usize)).collect();

        // Reference Occurrences
        let mut ro = vec![0; 26];
        for &c in &s1 {
            ro[c] += 1;
        }
        let ro = ro;

        // Actual Occurrences
        let mut ao = vec![0; 26];

        let mut beg = 0; // Inclusive
        let mut end = 0; // Exclusive

        while end < s2.len() {
            let c = s2[end];
            ao[c] += 1;
            end += 1;
            while ao[c] > ro[c] && beg < end {
                let c = s2[beg];
                ao[c] -= 1;
                beg += 1;
            }
            if end - beg == s1.len() {
                return true;
            }
        }
        return false;
    }
}
