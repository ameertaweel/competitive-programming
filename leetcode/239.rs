// LeetCode/239 - Sliding Window Maximum

use std::collections::{BTreeMap, VecDeque};

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;

        let mut dqi = VecDeque::new();
        let mut dqv = VecDeque::new();
        let mut ans = vec![];
        for i in 0..nums.len() {
            while dqv.len() > 0 && nums[i] >= dqv[dqv.len() - 1] {
                dqi.pop_back().unwrap();
                dqv.pop_back().unwrap();
            }
            dqi.push_back(i);
            dqv.push_back(nums[i]);
            if i >= k - 1 && dqi[0] < i - k + 1 {
                dqi.pop_front().unwrap();
                dqv.pop_front().unwrap();
            }
            if i >= k - 1 {
                ans.push(dqv[0]);
            }
        }

        return ans;
    }

    pub fn max_sliding_window_alt(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;

        let mut map = BTreeMap::new();

        for i in 0..(k - 1) {
            *map.entry(nums[i]).or_insert(0) += 1;
        }

        let mut ans = vec![];
        for i in (k - 1)..nums.len() {
            *map.entry(nums[i]).or_insert(0) += 1;
            let max = *map.last_key_value().unwrap().0;
            ans.push(max);
            if map[&nums[i - k + 1]] == 1 {
                map.remove(&nums[i - k + 1]);
            } else {
                *map.get_mut(&nums[i - k + 1]).unwrap() -= 1;
            }
        }

        return ans;
    }
}
