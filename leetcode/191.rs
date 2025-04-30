// LeetCode/191 - Number of 1 Bits

impl Solution {
    pub fn hamming_weight(n: i32) -> i32 {
        let mut ans = 0;
        for i in 0..32 {
            let mask = 1 << i;
            if n & mask > 0 {
                ans += 1;
            }
        }
        return ans;
    }
}
