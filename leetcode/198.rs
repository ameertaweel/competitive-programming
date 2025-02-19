use std::cmp::max;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        if nums.len() == 1 {
            return nums[0];
        }

        // Max cash at house i - 2
        let mut cash_2 = nums[0];
        // Max cash at house i - 1
        let mut cash_1 = max(nums[0], nums[1]);
        for c in &nums[2..] {
            let cash_0 = max(cash_2 + c, cash_1);
            cash_2 = cash_1;
            cash_1 = cash_0;
        }
        return cash_1;
    }
}
