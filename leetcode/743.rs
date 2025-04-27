// LeetCode/743 - Network Delay Time

use std::collections::BinaryHeap;

impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize - 1;

        let mut graph = vec![vec![]; n];
        for t in times {
            let (src, dst, cost) = (t[0] as usize - 1, t[1] as usize - 1, t[2]);
            graph[src].push((dst, cost));
        }
        let graph = graph;

        let mut h = BinaryHeap::new();
        h.push((0, k));

        let mut costs = vec![-1 as i32; n];
        costs[k] = 0;

        let mut visited = vec![false; n];

        while h.len() > 0 {
            let (cost, node) = h.pop().unwrap();
            let cost_curr = -1 * cost;

            if visited[node] {
                continue;
            }
            visited[node] = true;

            for &(dst, cost) in &graph[node] {
                let cost_next = cost_curr + cost;
                if costs[dst] == -1 || costs[dst] > cost_next {
                    h.push((-1 * cost_next, dst));
                    costs[dst] = cost_next;
                }
            }
        }

        let mut max = 0;
        for c in costs {
            if c == -1 {
                return -1;
            }
            if c > max {
                max = c;
            }
        }
        return max;
    }

    pub fn network_delay_time_alt(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize - 1;

        let mut graph = vec![vec![]; n];
        for t in times {
            let (src, dst, cost) = (t[0] as usize - 1, t[1] as usize - 1, t[2]);
            graph[src].push((dst, cost));
        }
        let graph = graph;

        let mut costs = vec![-1 as i32; n];
        costs[k] = 0;

        Self::network_delay_time_alt_helper(&graph, k, &mut costs, 0);

        let mut max = 0;
        for c in costs {
            if c == -1 {
                return -1;
            }
            if c > max {
                max = c;
            }
        }
        return max;
    }

    pub fn network_delay_time_alt_helper(
        graph: &Vec<Vec<(usize, i32)>>,
        k: usize,
        costs: &mut Vec<i32>,
        current_cost: i32,
    ) {
        costs[k] = current_cost;
        for &(dst, next_cost) in &graph[k] {
            let path_cost = current_cost + next_cost;
            if costs[dst] == -1 || costs[dst] > path_cost {
                Self::network_delay_time_alt_helper(graph, dst, costs, path_cost);
            }
        }
    }
}
