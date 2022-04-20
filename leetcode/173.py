# Definition for a binary tree node.
# class TreeNode:
#	 def __init__(self, val=0, left=None, right=None):
#		 self.val = val
#		 self.left = left
#		 self.right = right
class BSTIterator:

	def __init__(self, root: Optional[TreeNode]):
		self.root = root
		self.has_next = True
		
		def generator(node, all_right = True):
			if node:
				yield from generator(node.left, False)
				if all_right and not node.right:
					self.has_next = False
				yield node.val
				yield from generator(node.right, all_right and True)
		
		self.generator = generator(root)

	def next(self, level = 0) -> int:
		return self.generator.__next__()

	def hasNext(self) -> bool:
		return self.has_next


# Your BSTIterator object will be instantiated and called as such:
# obj = BSTIterator(root)
# param_1 = obj.next()
# param_2 = obj.hasNext()
