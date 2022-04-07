from bisect import bisect_left

class Solution:
	def lastStoneWeight(self, stones: List[int]) -> int:
		stones.sort()
		while len(stones) > 1:
			a = stones.pop()
			b = stones.pop()
			if a > b:
				stones.insert(bisect_left(stones, a - b), a - b)
		if len(stones) == 0:
			return 0
		return stones[0]
