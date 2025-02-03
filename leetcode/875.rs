impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut i = 1;
        let mut j = piles.iter().max().unwrap() + 1;

        let mut min_k = j - 1;

        while i < j {
            let mid = i + (j - i) / 2;

            let time: i32 = piles
                .iter()
                .map(|p| {
                    if p % mid == 0 {
                        return p / mid;
                    }
                    return p / mid + 1;
                })
                .sum();

            if time <= h {
                if mid < min_k {
                    min_k = mid;
                }
                j = mid;
                continue;
            }

            i = mid + 1;
            continue;
        }

        return min_k;
    }
}
