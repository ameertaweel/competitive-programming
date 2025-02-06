impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let n = prices.len();
        let mut max_profit = 0;
        let mut min_purchase = prices[0];
        for p in prices {
            if p <= min_purchase {
                min_purchase = p;
                continue;
            }
            let profit = p - min_purchase;
            if profit > max_profit {
                max_profit = profit;
            }
        }
        return max_profit;
    }
}
