// LeetCode/371 - Sum of Two Integers

impl Solution {
    pub fn get_sum(a: i32, b: i32) -> i32 {
        let mut carry = 0;
        let mut result = 0;
        for i in 0..32 {
            let ai = a & (1 << i);
            let bi = b & (1 << i);
            result |= ai ^ bi ^ carry;
            carry = ((ai & bi) | (ai & carry) | (bi & carry)) << 1;
        }
        return result;
    }
}
