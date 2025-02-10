/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode* left;
 *     struct TreeNode* right;
 * };
 */
struct TreeNode* lowestCommonAncestor(struct TreeNode* root, struct TreeNode* p, struct TreeNode* q) {
    int min = p->val > q->val ? q->val : p->val;
    int max = p->val > q->val ? p->val : q->val;

    if (root->val < min) return lowestCommonAncestor(root->right, p, q);
    if (root->val > max) return lowestCommonAncestor(root->left, p, q);
    return root;
}
