/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode* left;
 *     struct TreeNode* right;
 * };
 */

bool isValidBSTRange(struct TreeNode* root, int min, int max);
bool isValidBSTLess(struct TreeNode* root, int max);
bool isValidBSTMore(struct TreeNode* root, int min);

bool isValidBST(struct TreeNode* root) {
    return isValidBSTLess(root->left, root->val) && isValidBSTMore(root->right, root->val);
}

bool isValidBSTRange(struct TreeNode* root, int min, int max) {
    if (root == NULL) return true;
    return root->val > min && root->val < max
        && isValidBSTRange(root->left, min, root->val)
        && isValidBSTRange(root->right, root->val, max);
}

bool isValidBSTLess(struct TreeNode* root, int max) {
    if (root == NULL) return true;
    return root->val < max
        && isValidBSTLess(root->left, root->val)
        && isValidBSTRange(root->right, root->val, max);
}

bool isValidBSTMore(struct TreeNode* root, int min) {
    if (root == NULL) return true;
    return root->val > min
        && isValidBSTRange(root->left, min, root->val)
        && isValidBSTMore(root->right, root->val);
}
