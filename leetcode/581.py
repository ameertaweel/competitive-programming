class Solution:
	def findUnsortedSubarray(self, nums: List[int]) -> int:		
		sorted_nums = sorted(nums)
		
		beg = -1
		end = -2
		
		for i, (old, new) in enumerate(zip(nums, sorted_nums)):
			if old != new:
				if beg == -1:
					beg = i
				else:
					end = i
		
		return end - beg + 1
