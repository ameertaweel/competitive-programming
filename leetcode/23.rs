// LeetCode/23 - Merge k Sorted Lists

// NOTE: Very slow due to excessive cloning. The C++ version is faster.

use std::collections::BinaryHeap;

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut lists = lists;
        let mut heap = BinaryHeap::new();

        for i in 0..lists.len() {
            if let Some(l) = lists[i].clone() {
                heap.push((-1 * l.val, i));
                lists[i] = l.next;
            }
        }

        if heap.len() == 0 {
            return None;
        }

        let mut head = Some(Box::new(ListNode::new(0)));
        let mut ans = &mut head;

        while heap.len() > 0 {
            let (val, idx) = heap.pop().unwrap();
            ans.as_mut().unwrap().val = -1 * val;
            if let Some(l) = lists[idx].clone() {
                heap.push((-1 * l.val, idx));
                lists[idx] = l.next;
            }
            if heap.len() > 0 {
                let next = Some(Box::new(ListNode::new(0)));
                ans.as_mut().unwrap().next = next;
                ans = &mut ans.as_mut().unwrap().next;
            }
        }

        return head;
    }
}
