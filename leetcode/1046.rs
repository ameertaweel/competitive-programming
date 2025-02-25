// LeetCode/1046 - Last Stone Weight

use std::collections::BinaryHeap;

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut heap = BinaryHeap::from(stones);
        while heap.len() > 1 {
            let y = heap.pop().unwrap();
            let x = heap.pop().unwrap();

            if x == y {
                continue;
            }

            let smashed = y - x;
            heap.push(smashed);
        }
        return if heap.len() == 0 {
            0
        } else {
            heap.pop().unwrap()
        };
    }
}
