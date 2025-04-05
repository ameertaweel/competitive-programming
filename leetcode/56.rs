// LeetCode/56 - Merge Intervals

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        intervals.sort();
        let mut new_intervals = vec![];
        let (mut beg_curr, mut end_curr) = (intervals[0][0], intervals[0][1]);
        for i in 0..intervals.len() {
            let (beg_next, end_next) = (intervals[i][0], intervals[i][1]);
            if beg_next > end_curr {
                new_intervals.push(vec![beg_curr, end_curr]);
                beg_curr = beg_next;
                end_curr = end_next;
                continue;
            }
            if end_next > end_curr {
                end_curr = end_next;
            }
        }
        new_intervals.push(vec![beg_curr, end_curr]);
        return new_intervals;
    }
}
