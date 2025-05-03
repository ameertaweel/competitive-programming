// LeetCode/7 - Reverse Integer

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        if x == 0 {
            return 0;
        }
        let mut old = x as i32;
        let mut new = 0 as i32;
        while old != 0 {
            let digit = old % 10;
            match new.checked_mul(10) {
                Some(n) => {
                    new = n;
                }
                None => {
                    return 0;
                }
            }
            new += digit;
            old /= 10;
        }
        return new;
    }
}
