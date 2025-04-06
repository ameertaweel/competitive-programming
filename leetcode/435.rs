// LeetCode/435 - Non-Overlapping Intervals

impl Solution {
    pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        let n = intervals.len();

        let mut intervals = intervals;
        intervals.sort_unstable_by(|a, b| a[0].cmp(&b[0]));
        let intervals = intervals;

        let mut remove = 0;

        let mut end_prev = intervals[0][1];
        for i in 1..n {
            let (beg_curr, end_curr) = (intervals[i][0], intervals[i][1]);

            // No Overlap
            if beg_curr >= end_prev {
                end_prev = end_curr;
                continue;
            }

            // Overlap
            remove += 1;
            if end_prev <= end_curr {
                continue;
            }
            end_prev = end_curr;
        }

        return remove;
    }
}
