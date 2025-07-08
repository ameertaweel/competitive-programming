// LeetCode/50 - Pow(x, n)

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if n == 0 || x == 1.0 {
            return 1.0;
        }

        let neg = n < 0;
        let min = n == i32::MIN;
        let n = if min {
            -1 * (n + 1)
        } else if neg {
            -1 * n
        } else {
            n
        };

        let mut ans = x;
        let mut acc = x;
        let mut aci = 1;
        let mut i = 1;

        while i != n {
            if ans == 0.0 {
                return 0.0;
            }
            (acc, aci) = (acc * acc, aci * 2);
            if i + aci > n {
                (acc, aci) = (x, 1);
            }
            ans *= acc;
            i += aci;
        }

        return if min {
            1.0 / (ans * x)
        } else if neg {
            1.0 / ans
        } else {
            ans
        };
    }
}
