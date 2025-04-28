// LeetCode/1584 - Min Cost to Connect All Points

use std::collections::{BinaryHeap, HashSet};

const INF: i32 = (1e6 as i32 - -1e6 as i32).abs() + (1e6 as i32 - -1e6 as i32).abs();

impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();

        let mut dist = vec![INF; n];
        let mut visited = vec![false; n];
        let mut visited_count = 0;

        dist[0] = 0;
        let mut cost = 0;

        while visited_count < n {
            let mut i = 0;
            let mut c = INF;
            for j in 0..n {
                if visited[j] {
                    continue;
                }
                if dist[j] < c {
                    i = j;
                    c = dist[j];
                }
            }
            visited[i] = true;
            visited_count += 1;
            cost += c;
            let (x1, y1) = (points[i][0], points[i][1]);
            for j in 0..n {
                if visited[j] {
                    continue;
                }
                let (x2, y2) = (points[j][0], points[j][1]);
                let d = (x2 - x1).abs() + (y2 - y1).abs();
                if d < dist[j] {
                    dist[j] = d;
                }
            }
        }

        return cost;
    }

    pub fn min_cost_connect_points_alt(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();

        let mut nodes = HashSet::new();
        for i in 0..n {
            let (x, y) = (points[i][0], points[i][1]);
            nodes.insert((x, y));
        }

        let mut heap = BinaryHeap::new();
        heap.push((0, points[0][0], points[0][1]));

        let mut cost = 0;

        while nodes.len() > 0 {
            let (c, x, y) = heap.pop().unwrap();
            if !nodes.contains(&(x, y)) {
                continue;
            }
            nodes.remove(&(x, y));
            cost += -1 * c;
            for n in &nodes {
                let &(x_prime, y_prime) = n;
                let dist = (x_prime - x).abs() + (y_prime - y).abs();
                heap.push((-1 * dist, x_prime, y_prime));
            }
        }

        return cost;
    }
}
