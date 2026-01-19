// LeetCode/169 - Majority Element

class Solution {
    public int majorityElement(int[] nums) {
        int n = nums.length;

        if (n == 1) return nums[0];

        int a = 0;
        int b = 1;

        while (b < n) {
            if (nums[a] != nums[b]) {
                nums[a] = Integer.MIN_VALUE;
                nums[b] = Integer.MIN_VALUE;
                while (nums[a] == Integer.MIN_VALUE) a++;
                b = b <= a ? a + 1 : b + 1;
            } else {
                b++;
            }
        }

        return nums[a];
    }
}
