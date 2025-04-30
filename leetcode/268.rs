// LeetCode/268 - Missing Number

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut ideal = nums.len() as i32;
        let mut actual = 0;
        for i in 0..nums.len() {
            ideal ^= i as i32;
            actual ^= nums[i];
        }

        return ideal ^ actual;
    }
}
