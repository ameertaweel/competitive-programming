// LeetCode/26 - Remove Duplicates from Sorted Array

class Solution {
    public int removeDuplicates(int[] nums) {
        if (nums.length < 2) return nums.length;

        int j = 1;
        int last = nums[0];
        for (int i = 1; i < nums.length; i++) {
            if (nums[i] == last) continue;
            nums[j++] = nums[i];
            last = nums[i];
        }

        return j;
    }
}
