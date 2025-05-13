// LeetCode/518 - Coin Change II

impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let amount = amount as usize;

        let mut coins: Vec<_> = coins
            .iter()
            .map(|&c| c as usize)
            .filter(|&c| c <= amount)
            .collect();
        if coins.len() == 0 {
            return 1;
        }
        coins.sort_unstable();
        let coins = coins;

        let mut dp = vec![0; amount + 1];
        dp[0] = 1;

        for &c in &coins {
            for i in 1..=amount {
                if i >= c {
                    dp[i] += dp[i - c];
                }
            }
        }

        return dp[amount];
    }

    pub fn change_alt(amount: i32, coins: Vec<i32>) -> i32 {
        let amount = amount as usize;

        let mut coins: Vec<_> = coins
            .iter()
            .map(|&c| c as usize)
            .filter(|&c| c <= amount)
            .collect();
        if coins.len() == 0 {
            return 1;
        }
        coins.sort_unstable();
        let coins = coins;

        let mut dp = vec![vec![0; coins.len()]; amount + 1];

        for i in 1..=amount {
            let mut ways = 0;
            for j in 0..coins.len() {
                let c = coins[j];
                if i >= c {
                    ways += if c == i { 1 } else { dp[i - c][j] };
                }
                dp[i][j] = ways;
            }
        }

        return dp[amount][coins.len() - 1];
    }
}
