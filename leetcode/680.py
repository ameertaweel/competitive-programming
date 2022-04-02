class Solution:
	def isPalindrome(self, s: str, bad: bool = False) -> bool:
		i = 0
		j = len(s) - 1
		while (i < j):
			if s[i] != s[j]:
				if bad: return False
				return self.isPalindrome(s[i:j], True) or self.isPalindrome(s[i + 1:j + 1], True)
			i += 1
			j -= 1
				
		return True  

	def validPalindrome(self, s: str) -> bool:
		return self.isPalindrome(s, False)
