// LeetCode/53 - Maximum Subarray

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max = nums[0];
        let mut cur = if nums[0] > 0 { nums[0] } else { 0 };
        for i in 1..nums.len() {
            cur += nums[i];
            if cur > max {
                max = cur;
            }
            if cur < 0 {
                cur = 0;
            }
        }
        return max;
    }
}
