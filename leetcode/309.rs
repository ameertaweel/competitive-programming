// LeetCode/309 - Best Time to Buy and Sell Stock with Cooldown

use std::cmp::max;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let n = prices.len();

        // h1 -> Max profit if I go into day i + 1 holding stock
        // n1 -> Max profit if I go into day i + 1 not holding stock
        // n2 -> Max profit if I go into day i + 2 not holding stock
        let (mut h1, mut n1, mut n2) = (0, 0, 0);

        for i in (0..n).rev() {
            let h0 = max(n2 + prices[i], h1);
            let n0 = max(h1 - prices[i], n1);

            (h1, n1, n2) = (h0, n0, n1);
        }

        return n1; // We always start not holding stock
    }
}
