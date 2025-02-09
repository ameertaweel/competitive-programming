use std::collections::HashSet;

impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut fast = 0;
        let mut slow = 0;

        loop {
            fast = nums[nums[fast] as usize] as usize;
            slow = nums[slow] as usize;
            if fast == slow {
                break;
            }
        }

        // The fast and slow pointer will meet when the slow pointer's distance
        // to the duplicate is equal to the distance to the duplicate starting
        // from index 0.

        let mut slow_1 = slow;
        let mut slow_2 = 0 as usize;

        loop {
            slow_1 = nums[slow_1] as usize;
            slow_2 = nums[slow_2] as usize;

            if slow_1 == slow_2 {
                return slow_1 as i32;
            }
        }
    }

    pub fn find_duplicate_alt_1(nums: Vec<i32>) -> i32 {
        let n = nums.len() - 1;
        let mut seen = vec![false; n];
        for num in nums {
            if seen[(num - 1) as usize] {
                return num;
            }
            seen[(num - 1) as usize] = true;
        }
        panic!("No Duplicate");
    }

    pub fn find_duplicate_alt_2(nums: Vec<i32>) -> i32 {
        let mut seen = HashSet::new();
        for n in nums {
            if seen.contains(&n) {
                return n;
            }
            seen.insert(n);
        }
        panic!("No Duplicate");
    }
}
