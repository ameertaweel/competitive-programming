// LeetCode/1851 - Minimum Interval to Include Each Query

use std::collections::BinaryHeap;

impl Solution {
    pub fn min_interval(intervals: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let mut intervals = intervals;
        // Store the original index with each element before sorting.
        let mut queries: Vec<(i32, usize)> = queries
            .into_iter()
            .enumerate()
            .map(|(i, x)| (x, i))
            .collect();

        // Comparing tuples happens based on the first element.
        queries.sort_unstable();
        // Comparing vectors happens based on the first element.
        // This means intervals will be sorted by their starting point.
        intervals.sort_unstable();

        let queries = queries;
        let intervals = intervals;

        let mut heap = BinaryHeap::new();
        let mut ans = vec![-1; queries.len()];

        let mut i_idx = 0;
        for &(q, q_idx) in &queries {
            while i_idx < intervals.len() && intervals[i_idx][0] <= q {
                // (-1 * interval_length, interval_end)
                // Length multiplied by -1 to make it a min-heap
                heap.push((
                    -1 * (intervals[i_idx][1] - intervals[i_idx][0] + 1),
                    intervals[i_idx][1],
                ));
                i_idx += 1;
            }
            while (heap.len() > 0) {
                let &(length, end) = heap.peek().unwrap();
                if end < q {
                    let _ = heap.pop().unwrap();
                    continue;
                }
                ans[q_idx] = -1 * length;
                break;
            }
        }

        return ans;
    }
}
