# Definition for a binary tree node.
# class TreeNode:
#	 def __init__(self, val=0, left=None, right=None):
#		 self.val = val
#		 self.left = left
#		 self.right = right

def inOrderTraverseBST(root: Optional[TreeNode]):
	if root:
		yield from inOrderTraverseBST(root.left)
		yield root
		yield from inOrderTraverseBST(root.right)

class Solution:
	def recoverTree(self, root: Optional[TreeNode]) -> None:
		"""
		Do not return anything, modify root in-place instead.
		"""
		segments = [] # sorted segments (only 2 or 3 segments are possible)
		last_node = None

		for node in inOrderTraverseBST(root):
			if len(segments) == 0:
				if last_node and node.val < last_node.val:
					segments.append(last_node)
					segments.append(node) # first node in second segment
			elif node.val < last_node.val:
					segments.append(node) # first node in third segment
					break
			last_node = node
		
		segments[0].val, segments[-1].val = segments[-1].val, segments[0].val
