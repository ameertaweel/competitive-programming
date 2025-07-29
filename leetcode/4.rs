// LeetCode/4 - Median of Two Sorted Arrays

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (n1, n2) = (nums1.len(), nums2.len());
        if (n1 + n2) % 2 == 1 {
            return Self::find_num_at_index(&nums1, &nums2, (n1 + n2) / 2) as f64;
        }
        return (Self::find_num_at_index(&nums1, &nums2, (n1 + n2) / 2 - 1) as f64
            + Self::find_num_at_index(&nums1, &nums2, (n1 + n2) / 2) as f64)
            / 2.0;
    }

    pub fn find_num_at_index(nums1: &Vec<i32>, nums2: &Vec<i32>, index: usize) -> i32 {
        let (mut a, mut b) = (0, nums1.len());
        while a < b {
            let mid = a + (b - a) / 2;
            let i = mid + nums2.partition_point(|&x| x < nums1[mid]);
            let j = mid + nums2.partition_point(|&x| x <= nums1[mid]);
            if i <= index && index <= j {
                return nums1[mid];
            }
            if j < index {
                a = mid + 1;
            } else {
                b = mid;
            }
        }
        return Self::find_num_at_index(nums2, nums1, index);
    }
}
