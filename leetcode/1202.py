from bisect import insort

class Solution:
	def smallestStringWithSwaps(self, s: str, pairs: List[List[int]]) -> str:
		union_find = UnionFind(len(s))
		for i, j in pairs:
			union_find.union(i, j)
		
		elements = {}
		start = {}
		
		for i in range(len(s)):
			group = union_find.find(i)
			if group not in elements:
				elements[group] = []
				start[group] = 0
			insort(elements[group], s[i])
		
		ans = ""
		for i in range(len(s)):
			group = union_find.find(i)
			ans += elements[group][start[group]]
			start[group] += 1
		
		return ans


# UnionFind data structure
# With path compression and union by rank optimizations
class UnionFind:
	def __init__(self, size):
		self.root = [i for i in range(size)]
		self.rank = [1] * size

	def find(self, x):
		if x == self.root[x]:
			return x
		# Path compression
		self.root[x] = self.find(self.root[x])
		return self.root[x]

	def union(self, x, y):
		rootX = self.find(x)
		rootY = self.find(y)
		if rootX != rootY:
			# Union by rank
			if self.rank[rootX] > self.rank[rootY]:
				self.root[rootY] = rootX
			elif self.rank[rootX] < self.rank[rootY]:
				self.root[rootX] = rootY
			else:
				self.root[rootY] = rootX
				self.rank[rootX] += 1

	def connected(self, x, y):
		return self.find(x) == self.find(y)

