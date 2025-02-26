// LeetCode/215 - Kth Largest Element in an Array

use std::collections::BinaryHeap;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut heap = BinaryHeap::from(nums);
        let mut ans = 0;
        for _ in 0..k {
            ans = heap.pop().unwrap();
        }
        return ans;
    }
}
