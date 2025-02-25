// LeetCode/703 - Kth Largest Element in a Stream

use std::cmp::Reverse;
use std::collections::BinaryHeap;

/**
 * Your KthLargest object will be instantiated and called as such:
 * let obj = KthLargest::new(k, nums);
 * let ret_1: i32 = obj.add(val);
 */
struct KthLargest {
    k: usize,
    heap: BinaryHeap<Reverse<i32>>, // Min Heap
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let k = k as usize;
        let mut s = KthLargest {
            k,
            heap: BinaryHeap::from(nums.into_iter().map(|n| Reverse(n)).collect::<Vec<_>>()),
        };
        while s.heap.len() > k {
            s.heap.pop().unwrap();
        }
        return s;
    }

    fn add(&mut self, val: i32) -> i32 {
        if self.heap.len() < self.k - 1 {
            panic!("We don't have a kth element.");
        }
        if self.heap.len() == self.k - 1 {
            self.heap.push(Reverse(val));
            let kth = self.heap.peek().unwrap().0;
            return kth;
        }
        let kth = self.heap.peek().unwrap().0;
        if val < kth {
            return kth;
        }
        self.heap.pop().unwrap();
        self.heap.push(Reverse(val));
        let kth = self.heap.peek().unwrap().0;
        return kth;
    }
}
