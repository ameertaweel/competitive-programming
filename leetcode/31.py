from bisect import bisect_right

class Solution:
	def nextPermutation(self, nums: List[int]) -> None:
		"""
		Do not return anything, modify nums in-place instead.
		"""
		hand = []
		for i, n in reversed(list(enumerate(nums))):
			j = bisect_right(hand, n, key = lambda x: x[0])
			if j != len(hand):
				del nums[hand[j][1]]
				nums.insert(i, hand[j][0])
				nums[i + 1:] = sorted(nums[i+1:])
				return
			hand.append((n, i))
		nums.reverse()
