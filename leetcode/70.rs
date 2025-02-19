impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        // Number of possible ways to reach stair i - 2
        let mut ways_2 = 0;
        // Number of possible ways to reach stair i - 1
        let mut ways_1 = 1;

        for _ in 0..n {
            let ways_0 = ways_2 + ways_1;
            ways_2 = ways_1;
            ways_1 = ways_0;
        }

        return ways_1;
    }
}
