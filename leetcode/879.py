# Definition for a binary tree node.
# class TreeNode:
#	 def __init__(self, val=0, left=None, right=None):
#		 self.val = val
#		 self.left = left
#		 self.right = right
class Solution:
	def linearizeBST(self, root: TreeNode) -> (TreeNode, TreeNode):
		if root.left == None and root.right == None:
			return (root, root)
		
		start, end = root, root

		if root.left != None:
			left = self.linearizeBST(root.left)
			left[1].right = root
			root.left = None
			start = left[0]
		
		if root.right != None:
			right = self.linearizeBST(root.right)
			root.right = right[0]
			end = right[1]
		
		return (start, end)
		
	def increasingBST(self, root: TreeNode) -> TreeNode:
		return self.linearizeBST(root)[0]
