// LeetCode/45 - Jump Game II

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return 0;
        }

        let mut lim_curr = 0;
        let mut lim_next = 0;
        let mut jumps = 0;

        for i in 0..nums.len() {
            if i + nums[i] as usize > lim_next {
                lim_next = i + nums[i] as usize;
            }
            if lim_next >= nums.len() - 1 {
                return jumps + 1;
            }
            if i == lim_curr {
                jumps += 1;
                lim_curr = lim_next;
            }
        }

        panic!("Didn't Reach Last Index");
    }
}
