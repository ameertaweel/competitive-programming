/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode* left;
 *     struct TreeNode* right;
 * };
 */
int maxDepth(struct TreeNode* root) {
    if (root == NULL) return 0;

    int d_left = maxDepth(root->left);
    int d_right = maxDepth(root->right);
    int d_max = d_left > d_right ? d_left : d_right;

    return 1 + d_max;
}
