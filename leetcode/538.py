# Definition for a binary tree node.
# class TreeNode:
#	 def __init__(self, val=0, left=None, right=None):
#		 self.val = val
#		 self.left = left
#		 self.right = right
	
class Solution:
	def bstToGreater(self, root: Optional[TreeNode], start: int = 0) -> Optional[TreeNode]:
		if root == None:
			return 0

		rightVal = self.bstToGreater(root.right, start)
		noStartVal = rightVal + root.val
		root.val = start + noStartVal
		noStartVal += self.bstToGreater(root.left, root.val)
		
		return noStartVal
	
	def convertBST(self, root: Optional[TreeNode], sum: int = 0) -> Optional[TreeNode]:
		self.bstToGreater(root)
		return root
