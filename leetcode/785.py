class Solution:
	def __init__(self):
		self.visited = []
		self.step = []
		self.i = -1

	def isBipartite(self, graph: List[List[int]]) -> bool:
		self.visited = [False] * len(graph)
		self.step = [-1] * len(graph)
		
		def hasOddCycle(node: int = 0, parent: int = -1) -> bool:
			if self.visited[node]:
				return False

			self.i += 1
			self.visited[node] = True
			self.step[node] = self.i
			
			for neighbor in graph[node]:
				if neighbor == parent:
					continue

				if self.visited[neighbor]:
					if (self.step[node] - self.step[neighbor]) % 2 == 0:
						return True
					continue
				
				if hasOddCycle(neighbor, node):
					return True
				
				self.i -= 1
			
			return False
		
		for node in range(len(graph)):
			self.i = -1
			if hasOddCycle(node):
				return False
		
		return True
