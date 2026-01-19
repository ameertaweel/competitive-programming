// LeetCode/80 - Remove Duplicates from Sorted Array II

class Solution {
    public int removeDuplicates(int[] nums) {
        if (nums.length < 2) return nums.length;

        int j = 1;
        int last = nums[0];
        int reps = 1;

        for (int i = 1; i < nums.length; i++) {
            if (nums[i] == last) {
                reps++;
                if (reps > 2) continue;
                nums[j++] = nums[i];
            } else {
                last = nums[i];
                reps = 1;
                nums[j++] = nums[i];
            }
        }

        return j;
    }
}
