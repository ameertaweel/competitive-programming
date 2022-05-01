from collections import defaultdict

class Solution:
	def minimumCardPickup(self, cards: List[int]) -> int:
		seen = defaultdict(lambda : -1)
		min_matching = -1
		for i, card in enumerate(cards):
			if seen[card] != -1:
				distance = i - seen[card] + 1
				if min_matching == -1 or distance < min_matching:
					min_matching = distance
			seen[card] = i
		return min_matching
