impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let chars: Vec<_> = s.chars().map(|c| c as usize).collect();
        let mut i = 0;
        let mut j = 0;
        let mut max_len = 0;
        let mut seen = vec![false; 255];

        while j < chars.len() {
            while seen[chars[j]] {
                seen[chars[i]] = false;
                i += 1;
            }

            seen[chars[j]] = true;
            j += 1;
            let len = j - i;
            if len > max_len {
                max_len = len;
            }
        }

        return max_len as i32;
    }
}
