from heapq import heappush, heappop

class Solution:
	def find132pattern(self, nums: List[int]) -> bool:
		min = []
		heap = []
		
		for i, n in enumerate(nums):
			if i == 0 or n < min[i - 1]:
				min.append(n)
			else:
				min.append(min[i - 1])
				
		for i, j in zip(reversed(min), reversed(nums)):
			if j > i:
				while len(heap) > 0:
					if heap[0] >= j:
						break
					if heap[0] > i:
						return True
					heappop(heap)
			heappush(heap, j)
		
		return False
