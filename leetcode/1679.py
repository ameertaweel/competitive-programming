from collections import defaultdict
class Solution:
	def maxOperations(self, nums: List[int], k: int) -> int:
		count = defaultdict(lambda : 0)
		ans = 0
		for n in nums:
			if n >= k:
				continue
			if count[k - n] > 0:
				count[k - n] -= 1
				ans += 1
			else:
				count[n] += 1
		
		return ans
