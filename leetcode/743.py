import bisect
from collections import defaultdict

class Solution:
	def networkDelayTime(self, times: List[List[int]], n: int, k: int) -> int:
		neighbors = defaultdict(lambda : [])
		
		for src, dst, time in times:
			neighbors[src].append((dst, time))
		
		visited = defaultdict(lambda : False)
		
		total_time = 0
		total_visited = 0
		
		edges = [(k, 0)]
		while len(edges) > 0:
			# acc => accumulated time
			node, acc = edges.pop(0)
			
			if visited[node]:
				continue
			
			visited[node] = True
			total_visited += 1
			
			if acc > total_time:
				total_time = acc
			
			for neighbor, neighbor_time in neighbors[node]:
				bisect.insort(edges, (neighbor, acc + neighbor_time), key = lambda x : x[1])
		
		return total_time if total_visited == n else -1
