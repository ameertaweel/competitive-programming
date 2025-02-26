// LeetCode/621 - Task Scheduler

use std::collections::{BinaryHeap, VecDeque};

impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut counts = vec![0; 26];
        for t in tasks {
            let idx = t as usize - 'A' as usize;
            counts[idx] += 1;
        }
        let counts = counts.into_iter().filter(|&c| c > 0).collect::<Vec<_>>();

        let mut scheduled = BinaryHeap::from(counts);
        let mut idle = VecDeque::new();
        let mut intervals = 0;

        while scheduled.len() + idle.len() > 0 {
            intervals += 1;
            match scheduled.pop() {
                Some(1) => {}
                Some(priority) => idle.push_back((priority - 1, intervals)),
                None => {}
            }
            if idle.len() > 0 && intervals - idle[0].1 >= n {
                let (priority, _) = idle.pop_front().unwrap();
                scheduled.push(priority);
            }
        }

        return intervals;
    }
}
