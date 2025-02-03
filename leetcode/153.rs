impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut j = nums.len();

        while i < j {
            let mid = i + (j - i) / 2;
            println!("{}, {}, {}", i, mid, j);

            if mid > 0 && nums[mid - 1] > nums[mid] {
                return nums[mid];
            }

            if nums[i] > nums[mid] {
                j = mid;
                continue;
            }
            if nums[mid] > nums[j - 1] {
                i = mid + 1;
                continue;
            }

            return nums[i];
        }

        return nums[0];
    }
}
