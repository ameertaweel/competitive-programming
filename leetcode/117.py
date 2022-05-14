"""
# Definition for a Node.
class Node:
	def __init__(self, val: int = 0, left: 'Node' = None, right: 'Node' = None, next: 'Node' = None):
		self.val = val
		self.left = left
		self.right = right
		self.next = next
"""

def traverse(root):
	"""
	BFS on the binary tree, with right child coming always first.
	"""
	
	queue = [(root, 0)] # int is depth
	max_depth = -1
	
	while len(queue) > 0:
		node, depth = queue.pop(0)
		
		if depth > max_depth:
			yield node, True
			max_depth = depth
		else:
			yield node, False

		if node != None:
			queue.append((node.right, depth + 1))
			queue.append((node.left, depth + 1))

class Solution:
	def connect(self, root: 'Node') -> 'Node':   
		next = None
		for i, (node, is_new_level) in enumerate(traverse(root)):
			if is_new_level:
				next = None
			
			if node != None:
				node.next = next
				next = node
				
		return root
