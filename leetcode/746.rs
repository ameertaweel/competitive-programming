use std::cmp::min;

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        // Min cost at step i - 2
        let mut cost_2 = cost[0];
        // Min cost at step i - 1
        let mut cost_1 = cost[1];
        for c in &cost[2..] {
            let cost_0 = c + min(cost_2, cost_1);
            cost_2 = cost_1;
            cost_1 = cost_0;
        }
        return min(cost_2, cost_1);
    }
}
