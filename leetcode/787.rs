// LeetCode/787 - Cheapest Flights Within K Stops

use std::collections::VecDeque;

impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let (n, src, dst, k) = (n as usize, src as usize, dst as usize, k as usize);

        let graph = {
            let mut g = vec![vec![]; n];
            for f in flights {
                let (src, dst, cost) = (f[0] as usize, f[1] as usize, f[2]);
                g[src].push((dst, cost));
            }
            g
        };

        let mut costs = vec![-1; n];
        let mut q = VecDeque::new();

        costs[src] = 0;
        q.push_back((src, 0, 0));

        let mut ans = -1;

        while q.len() > 0 {
            let (node, stops, cost) = q.pop_front().unwrap();

            if node == dst {
                if ans == -1 || cost < ans {
                    ans = cost;
                }
                continue;
            }

            if stops == k + 1 {
                continue;
            }

            for &(d, c) in &graph[node] {
                if costs[d] != -1 && cost + c >= costs[d] {
                    continue;
                }
                costs[d] = cost + c;
                q.push_back((d, stops + 1, cost + c));
            }
        }

        return ans;
    }
}
