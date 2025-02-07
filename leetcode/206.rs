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
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut ansr = None;
        let mut curr = &head;

        while *curr != Option::None {
            ansr = Option::Some(Box::new(ListNode {
                val: curr.as_ref().unwrap().val,
                next: ansr,
            }));
            curr = &curr.as_ref().unwrap().next;
        }

        return ansr;
    }
}
