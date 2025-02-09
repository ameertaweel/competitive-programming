/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode* left;
 *     struct TreeNode* right;
 * };
 */

typedef struct TreeNode TN;

struct DepthAndDiameter {
    int depth;
    int diam;
};
typedef struct DepthAndDiameter DnD;

DnD diameterOfBinaryTreeHelper(TN* root);

int diameterOfBinaryTree(TN* root) {
    DnD dnd = diameterOfBinaryTreeHelper(root);
    return dnd.diam;    
}

DnD diameterOfBinaryTreeHelper(TN* root) {
    if (root == NULL) return (DnD) { 0, 0 };
    if (root->left == NULL && root->right == NULL) return (DnD) { 1, 0 };

    DnD l = diameterOfBinaryTreeHelper(root->left);
    DnD r = diameterOfBinaryTreeHelper(root->right);

    // Longest path passing through the root node
    int diam_root = l.depth + r.depth;

    int depth = (l.depth > r.depth ? l.depth : r.depth) + 1;

    int diam = l.diam > r.diam ? l.diam : r.diam;
    if (diam_root > diam) diam = diam_root;

    return (DnD) { depth, diam };
}
