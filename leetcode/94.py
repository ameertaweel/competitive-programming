# Definition for a binary tree node.
# class TreeNode:
#	 def __init__(self, val=0, left=None, right=None):
#		 self.val = val
#		 self.left = left
#		 self.right = right
class Solution:
	# Use iterative approach
	def inorderTraversal(self, root: Optional[TreeNode]) -> List[int]:
		if not root:
			return []
		ans = []
		stack = []
		n = root
		while True:
			while n:
				stack.append(n)
				n = n.left
			n = stack.pop()
			ans.append(n.val)
			
			if n.right:
				n = n.right
			elif len(stack) == 0:
				return ans
			else:
				n = None
