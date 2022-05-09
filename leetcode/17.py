class Solution:
	def __init__(self):
		self.digit_values = {
			"2": ["a", "b", "c"],
			"3": ["d", "e", "f"],
			"4": ["g", "h", "i"],
			"5": ["j", "k", "l"],
			"6": ["m", "n", "o"],
			"7": ["p", "q", "r", "s"],
			"8": ["t", "u", "v"],
			"9": ["w", "x", "y", "z"]
		}


	def letterCombinations(self, digits: str) -> List[str]:
		if len(digits) == 0:
			return []
		
		digit = digits[0]
		if len(digits) == 1:
			return self.digit_values[digit]
		
		reduced = self.letterCombinations(digits[1:])
		ans = []
		
		for r in reduced:
			for v in self.digit_values[digit]:
				ans.append(v + r)
		
		return ans
