/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode* left;
 *     struct TreeNode* right;
 * };
 */

bool isEqual(struct TreeNode* a, struct TreeNode* b);

bool isSubtree(struct TreeNode* root, struct TreeNode* subRoot) {
    if (root == NULL || subRoot == NULL) return root == subRoot;

    return isEqual(root, subRoot) || isSubtree(root->left, subRoot) || isSubtree(root->right, subRoot);
}

bool isEqual(struct TreeNode* a, struct TreeNode* b) {
    if (a == NULL || b == NULL) return a == b;

    return a->val == b->val && isEqual(a->left, b->left) && isEqual(a->right, b->right);
}
