# Definition for a binary tree node.
# class TreeNode:
#	 def __init__(self, val=0, left=None, right=None):
#		 self.val = val
#		 self.left = left
#		 self.right = right

def traverseBinaryTree(root, depth = 0):
	if root != None:
		yield root.val, depth
		yield from traverseBinaryTree(root.left, depth + 1)
		yield from traverseBinaryTree(root.right, depth + 1)

class Solution:
	def deepestLeavesSum(self, root: Optional[TreeNode]) -> int:
		ans = 0
		max_depth = 0
		
		for n, depth in traverseBinaryTree(root):
			if depth == max_depth:
				ans += n
			elif depth > max_depth:
				ans = n
				max_depth = depth
		
		return ans
