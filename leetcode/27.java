// LeetCode/27 - Remove Element

class Solution {
    public int removeElement(int[] nums, int val) {
        int n = nums.length;

        if (n == 0) return 0;

        int a = 0;
        int b = n - 1;

        while (a <  n && nums[a] != val) a++;
        while (b >= 0 && nums[b] == val) b--;

        while (a < n && b >= 0 && a < b) {
            int temp = nums[b];
            nums[b] = nums[a];
            nums[a] = temp;

            while (nums[a] != val) a++;
            while (nums[b] == val) b--;
        }

        int k = b + 1;
        return k;
    }
}
