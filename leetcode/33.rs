// Binary Search
pub fn search(nums: &[i32], target: i32) -> i32 {
    let mut i = 0;
    let mut j = nums.len();

    while i < j {
        let mid = i + (j - i) / 2;

        if nums[mid] == target {
            return mid as i32;
        }

        if nums[mid] < target {
            i = mid + 1;
            continue;
        }

        if nums[mid] > target {
            j = mid;
            continue;
        }
    }

    return -1;
}

// Find Rotation Index
pub fn find_min(nums: &[i32]) -> usize {
    let mut i = 0;
    let mut j = nums.len();

    while i < j {
        let mid = i + (j - i) / 2;

        if mid > 0 && nums[mid - 1] > nums[mid] {
            return mid;
        }

        if nums[i] > nums[mid] {
            j = mid;
            continue;
        }
        if nums[mid] > nums[j - 1] {
            i = mid + 1;
            continue;
        }

        return i;
    }

    return 0;
}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let rotate_idx = find_min(&nums);

        let res = search(&nums[rotate_idx..], target);

        if rotate_idx == 0 || res != -1 {
            return rotate_idx as i32 + res;
        }

        return search(&nums[0..rotate_idx], target);
    }
}
