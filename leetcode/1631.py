import math, heapq

class Solution:
	def minimumEffortPath(self, heights: List[List[int]]) -> int:
		# Using Dijkstra's Algorithm
		
		# Constants
		rows = len(heights)
		cols = len(heights[0])
		dirs = ((1, 0), (-1, 0), (0, 1), (0, -1))
		
		# Initializations
		moves = []		
		efforts = [[math.inf] * cols for _ in range(rows)] # 2D matrix
		
		heapq.heappush(moves, (0, 0, 0))
		
		while len(moves) > 0:
			effort, i, j = heapq.heappop(moves)
			
			if effort >= efforts[i][j]:
				continue
			efforts[i][j] = effort
			
			if i == rows - 1 and j == cols - 1:
				continue
				
			for delta_i, delta_j in dirs:
				new_i = i + delta_i
				new_j = j + delta_j
				if new_i < 0 or new_i >= rows or new_j < 0 or new_j >= cols:
					continue
				new_effort = max(effort, abs(heights[i][j] - heights[new_i][new_j]))
				heapq.heappush(moves, (new_effort, new_i, new_j))
			
		return efforts[-1][-1]
