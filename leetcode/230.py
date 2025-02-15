# Definition for a binary tree node.
# class TreeNode:
# 	 def __init__(self, val=0, left=None, right=None):
# 		 self.val = val
# 		 self.left = left
# 		 self.right = right


class Solution:
    def kthSmallest(self, root: Optional[TreeNode], k: int) -> int:
        for i, item in enumerate(inOrderTraverseBST(root)):
            if i == (k - 1):
                return item

    def kthSmallest_alt(self, root: Optional[TreeNode], k: int) -> int:
        size, ans = kthSmallestHelper(root, k)
        return ans


def inOrderTraverseBST(root: Optional[TreeNode]):
    if root:
        yield from inOrderTraverseBST(root.left)
        yield root.val
        yield from inOrderTraverseBST(root.right)


def kthSmallestHelper(root: Optional[TreeNode], k: int) -> tuple[int, int]:
    if root == None:
        return (0, 0)

    l_size, l_ans = kthSmallestHelper(root.left, k)

    if l_size == k:
        return (l_size, l_ans)
    if l_size == k - 1:
        return (k, root.val)

    k_prime = k - (l_size + 1)

    r_size, r_ans = kthSmallestHelper(root.right, k_prime)

    if r_size == k_prime:
        return (k, r_ans)

    return (l_size + r_size + 1, 0)
