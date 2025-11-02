// LeetCode/295 - Find Median from Data Stream

use std::collections::BinaryHeap;

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */
struct MedianFinder {
    less: BinaryHeap<i32>,
    more: BinaryHeap<i32>,
    median: Option<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    fn new() -> Self {
        MedianFinder {
            less: BinaryHeap::new(),
            more: BinaryHeap::new(),
            median: None,
        }
    }

    fn add_num(&mut self, num: i32) {
        if self.less.len() + self.more.len() == 0 && self.median == None {
            self.median = Some(num);
            return;
        }

        match self.median {
            Some(median) => {
                // Odd Number of Elements
                if num > median {
                    self.more.push(num * -1);
                    self.median = None;
                    self.less.push(median);
                } else if num < median {
                    self.less.push(num);
                    self.median = None;
                    self.more.push(median * -1);
                } else {
                    self.less.push(median);
                    self.median = None;
                    self.more.push(median * -1);
                }
            }
            None => {
                // Even Number of Elements
                let immediate_more = *self.more.peek().unwrap() * -1;
                let immediate_less = *self.less.peek().unwrap();
                if num > immediate_more {
                    self.median = Some(immediate_more);
                    self.more.pop();
                    self.more.push(num * -1);
                } else if num < immediate_less {
                    self.median = Some(immediate_less);
                    self.less.pop();
                    self.less.push(num);
                } else {
                    self.median = Some(num);
                }
            }
        }
    }

    fn find_median(&self) -> f64 {
        return match self.median {
            Some(median) => median as f64,
            None => {
                (((self.less.peek().unwrap()) + (self.more.peek().unwrap() * -1)) as f64)
                    / (2 as f64)
            }
        };
    }
}
