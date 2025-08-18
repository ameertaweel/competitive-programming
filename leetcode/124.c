// LeetCode/124 - Binary Tree Maximum Path Sum

/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode *left;
 *     struct TreeNode *right;
 * };
 */

int max;

int maxPathSumHelper(struct TreeNode* root) {
    if (root == NULL) return -1001;
    int lval = maxPathSumHelper(root -> left);
    int rval = maxPathSumHelper(root -> right);
    {
        int ans = root -> val;
        if (lval > 0) ans += lval;
        if (rval > 0) ans += rval;
        if (ans > max) max = ans;
    }
    int ans = root -> val;
    if (lval <= 0 && rval <= 0) return ans;
    ans += lval > rval ? lval : rval;
    return ans;
}

int maxPathSum(struct TreeNode* root) {
    max = -1001;
    maxPathSumHelper(root);
    return max;
}
