/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode* left;
 *     struct TreeNode* right;
 * };
 */

int goodNodesHelper(struct TreeNode* root, int max);

int goodNodes(struct TreeNode* root) {
    if (root == NULL) return 0;
    return goodNodesHelper(root, root->val);
}

int goodNodesHelper(struct TreeNode* root, int max) {
    if (root == NULL) return 0;
    if (root->val >= max) {
        return 1 + goodNodesHelper(root->left, root->val) + goodNodesHelper(root->right, root->val);
    }
    return goodNodesHelper(root->left, max) + goodNodesHelper(root->right, max);
}
