/**
 * Note: The returned array must be malloced, assume caller calls free().
 */
int* getConcatenation(int* nums, int numsSize, int* returnSize) {
    *returnSize = 2 * numsSize;
    int *ans = malloc(*returnSize * sizeof(*ans));
    for (int i = 0; i < numsSize; i++) {
        ans[i] = ans[i + numsSize] = nums[i];
    }
    return ans;
}
