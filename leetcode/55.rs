// LeetCode/55 - Jump Game

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut reach = 0;
        for i in 0..nums.len() {
            if i > reach {
                return false;
            }
            let reach_new = i + nums[i] as usize;
            if reach_new > reach {
                reach = reach_new;
            }
        }
        return true;
    }
}
