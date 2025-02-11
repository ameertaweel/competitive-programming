impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        // Number of ways to reach index i - 2
        let mut way_2 = 0;
        // Number of ways to reach index i - 1
        let mut way_1 = 1;
        // Digit at index i - 1
        let mut dig_1 = 9;

        for (i, c) in s.chars().enumerate() {
            let dig_0 = c.to_digit(10).unwrap();
            let way_0 = match dig_0 {
                0 => {
                    if dig_1 != 1 && dig_1 != 2 {
                        return 0;
                    } else {
                        way_2
                    }
                }
                1..=6 => {
                    if dig_1 == 1 || dig_1 == 2 {
                        way_2 + way_1
                    } else {
                        way_1
                    }
                }
                7..=9 => {
                    if dig_1 == 1 {
                        way_2 + way_1
                    } else {
                        way_1
                    }
                }
                _ => panic!("Invalid State"),
            };
            way_2 = way_1;
            way_1 = way_0;
            dig_1 = dig_0;
        }

        return way_1;
    }
}
