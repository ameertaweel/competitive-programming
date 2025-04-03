// LeetCode/57 - Insert Interval

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        if intervals.len() == 0 {
            return vec![new_interval];
        }

        let (beg_new, end_new) = (new_interval[0], new_interval[1]);

        let pos = match (intervals.binary_search_by(|i| i[0].cmp(&beg_new))) {
            Err(pos) => pos,
            Ok(pos) => pos,
        };

        let mut intervals = intervals;
        intervals.insert(pos, new_interval);
        let intervals = intervals;

        let mut new_intervals = vec![];
        let (mut beg_curr, mut end_curr) = (intervals[0][0], intervals[0][1]);

        for i in 1..intervals.len() {
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
