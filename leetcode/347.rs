use std::collections::HashMap;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut h = HashMap::new();

        for n in nums {
            *h.entry(n).or_insert(0) += 1;
        }

        let mut v: Vec<_> = h.iter().map(|(&key, &val)| (key, val)).collect();
        // Sort by count (descending)
        v.sort_by(|a, b| b.1.cmp(&a.1));

        return v.into_iter().take(k).map(|(key, _)| key).collect();
    }
}
