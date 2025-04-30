// LeetCode/338 - Counting Bits

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut ans = vec![0; n + 1];
        for i in 1..=n {
            ans[i] = if i % 2 == 0 {
                ans[i & (i - 1)] + 1
            } else {
                ans[i - 1] + 1
            };
        }
        return ans;
    }
}
