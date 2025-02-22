// dp[amount] = min {
//     dp[amount - coin_1] + 1,
//     dp[amount - coin_2] + 1,
//     ...,
//     dp[amount - coin_N] + 1,
// }

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        if amount == 0 {
            return 0;
        }
        let amount = amount as usize;
        let mut coins: Vec<usize> = coins
            .iter()
            .map(|&c| c as usize)
            .filter(|&c| c <= amount)
            .collect();
        coins.sort_unstable();
        coins.dedup();
        let coins = coins;
        if coins.len() == 0 {
            return -1;
        }
        let max = *coins.iter().max().unwrap();

        let mut dp = vec![amount + 1; max + amount];
        // Element at index max - 1 represents amount 0
        dp[max - 1] = 0;

        let mut len = max;
        for _ in 1..=amount {
            let coins_needed = coins.iter().map(|&c| dp[len - c]).min().unwrap() + 1;
            dp[len] = coins_needed;
            len += 1;
        }

        return if dp[len - 1] <= amount {
            dp[len - 1] as i32
        } else {
            -1
        };
    }
}
