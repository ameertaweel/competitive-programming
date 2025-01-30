use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let s: HashSet<_> = nums.into_iter().collect();
        let mut longest = 0;
        for &n in &s {
            if s.contains(&(n - 1)) {
                continue;
            }
            let mut len = 1;
            while s.contains(&(n + len)) {
                len += 1;
            }
            if len > longest {
                longest = len;
            }
        }
        return longest;
    }

    pub fn longest_consecutive_alt(nums: Vec<i32>) -> i32 {
        let mut seqs = HashMap::new();
        let mut seen = HashSet::new();

        for n in nums {
            if seen.contains(&n) {
                continue;
            }
            seen.insert(n);

            let is_suffix = seqs.contains_key(&(n - 1));
            let is_prefix = seqs.contains_key(&(n + 1));

            if is_suffix && is_prefix {
                // Join Two Sequences
                let a_beg = seqs[&(n - 1)];
                let a_end = n - 1;
                let b_beg = n + 1;
                let b_end = seqs[&(n + 1)];
                if a_beg != a_end {
                    seqs.remove(&a_end);
                }
                if b_beg != b_end {
                    seqs.remove(&b_beg);
                }
                seqs.insert(a_beg, b_end);
                seqs.insert(b_end, a_beg);
            } else if is_suffix {
                // Extend Sequence From End
                let beg = seqs[&(n - 1)];
                let end = n - 1;
                if beg != end {
                    seqs.remove(&end);
                }
                seqs.insert(beg, n);
                seqs.insert(n, beg);
            } else if is_prefix {
                // Extend Sequence From Beg
                let beg = n + 1;
                let end = seqs[&(n + 1)];
                if beg != end {
                    seqs.remove(&beg);
                }
                seqs.insert(n, end);
                seqs.insert(end, n);
            } else {
                seqs.insert(n, n);
            }
        }

        let mut max_seq_len = 0;

        for (beg, end) in &seqs {
            if beg > end {
                continue;
            }
            let seq_len = end - beg + 1;
            if seq_len > max_seq_len {
                max_seq_len = seq_len;
            }
        }

        return max_seq_len;
    }
}
