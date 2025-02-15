# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right


class Solution:
    def buildTree(self, preorder: List[int], inorder: List[int]) -> Optional[TreeNode]:
        n = len(preorder)
        return buildTreeHelper(preorder, inorder, 0, 0, n)


def buildTreeHelper(
    preorder: List[int],
    inorder: List[int],
    preorder_idx: int,
    inorder_idx: int,
    size: int,
) -> Optional[TreeNode]:
    if size == 0:
        return None

    root = preorder[preorder_idx]

    # Left Sub-Tree Size
    l_size = None
    for i in range(inorder_idx, inorder_idx + size):
        if inorder[i] == root:
            l_size = i - inorder_idx
            break

    left = buildTreeHelper(preorder, inorder, preorder_idx + 1, inorder_idx, l_size)
    right = buildTreeHelper(
        preorder,
        inorder,
        preorder_idx + 1 + l_size,
        inorder_idx + 1 + l_size,
        size - l_size - 1,
    )

    return TreeNode(root, left, right)
