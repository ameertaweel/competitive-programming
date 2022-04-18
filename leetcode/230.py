# Definition for a binary tree node.
# class TreeNode:
#	 def __init__(self, val=0, left=None, right=None):
#		 self.val = val
#		 self.left = left
#		 self.right = right

def inOrderTraverseBST(root: Optional[TreeNode]):
	if root:
		yield from inOrderTraverseBST(root.left)
		yield root.val
		yield from inOrderTraverseBST(root.right)

class Solution:
	def kthSmallest(self, root: Optional[TreeNode], k: int) -> int:
		for i, item in enumerate(inOrderTraverseBST(root)):
			if i == (k - 1):
				return item
