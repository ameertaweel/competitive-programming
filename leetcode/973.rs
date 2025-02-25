// LeetCode/973 - K Closest Points to Origin

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    // Time: O(N + k log N)
    // Space: O(N)
    pub fn k_closest(mut points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let points = points
            .into_iter()
            .map(|p| (Reverse(dist_to_origin_sqaure(p[0], p[1])), p))
            .collect::<Vec<_>>();
        let mut heap = BinaryHeap::from(points);
        let mut ans = vec![];
        for _ in 0..k {
            ans.push(heap.pop().unwrap().1);
        }
        return ans;
    }

    // Time: O(N log N)
    // Space: O(1)
    pub fn k_closest_alt(mut points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        points.sort_unstable_by(|a, b| {
            dist_to_origin_sqaure(a[0], a[1]).cmp(&dist_to_origin_sqaure(b[0], b[1]))
        });
        points.truncate(k);
        return points;
    }
}

fn dist_to_origin_sqaure(x: i32, y: i32) -> i32 {
    return x * x + y * y;
}
