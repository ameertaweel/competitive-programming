# """
# This is the interface that allows for creating nested lists.
# You should not implement it, or speculate about its implementation
# """
#class NestedInteger:
#	def isInteger(self) -> bool:
#		"""
#		@return True if this NestedInteger holds a single integer, rather than a nested list.
#		"""
#
#	def getInteger(self) -> int:
#		"""
#		@return the single integer that this NestedInteger holds, if it holds a single integer
#		Return None if this NestedInteger holds a nested list
#		"""
#
#	def getList(self) -> [NestedInteger]:
#		"""
#		@return the nested list that this NestedInteger holds, if it holds a nested list
#		Return None if this NestedInteger holds a single integer
#		"""

class NestedIterator:
	def __init__(self, nestedList: [NestedInteger]):		
		def generator(lst):
			for e in lst:
				if e.isInteger():
					yield e.getInteger()
				else:
					yield from generator(e.getList())
		
		self.flat_list = []
		
		for i in generator(nestedList):
			self.flat_list.append(i)
		
		self.has_next = len(self.flat_list) > 0
		
	
	def next(self) -> int:
		value = self.flat_list.pop(0)
		self.has_next = len(self.flat_list) > 0
		return value
		
	
	def hasNext(self) -> bool:
		return self.has_next
		 

# Your NestedIterator object will be instantiated and called as such:
# i, v = NestedIterator(nestedList), []
# while i.hasNext(): v.append(i.next())
