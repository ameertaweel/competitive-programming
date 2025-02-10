/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode* left;
 *     struct TreeNode* right;
 * };
 */

struct DepthBalanced {
    int depth;
    bool balanced;
};
typedef struct DepthBalanced DB;

DB isBalancedHelper(struct TreeNode* root);

bool isBalanced(struct TreeNode* root) {
    if (root == NULL) return true;

    DB db = isBalancedHelper(root);

    return db.balanced;
}

DB isBalancedHelper(struct TreeNode* root) {
    if (root == NULL) return (DB) { 0, true };

    DB l = isBalancedHelper(root->left);
    DB r = isBalancedHelper(root->right);

    int max_child_depth = l.depth > r.depth ? l.depth : r.depth;
    int min_child_depth = l.depth > r.depth ? r.depth : l.depth;

    int depth = max_child_depth + 1;
    bool balanced = l.balanced && r.balanced && (max_child_depth - min_child_depth <= 1);

    return (DB) { depth, balanced };
}
