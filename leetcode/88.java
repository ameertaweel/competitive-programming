// LeetCode/88 - Merge Sorted Array

class Solution {
    public void merge(int[] nums1, int m, int[] nums2, int n) {
        // Shift the elements in `nums1` to the end of `nums1`.
        for (int i = m - 1; i >= 0; i--) {
            nums1[n + i] = nums1[i];
        }

        int pr = 0;
        int p1 = n;
        int p2 = 0;

        while (p1 < n + m && p2 < n) {
            if (nums1[p1] <= nums2[p2]) {
                nums1[pr++] = nums1[p1++];
            } else {
                nums1[pr++] = nums2[p2++];
            }
        }

        while (p1 < n + m) nums1[pr++] = nums1[p1++];
        while (p2 < n)     nums1[pr++] = nums2[p2++];
    }
}
