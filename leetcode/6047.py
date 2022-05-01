class Solution:
	def removeDigit(self, number: str, digit: str) -> str:
		ans = 0
		for i in range(len(number)):
			if number[i] != digit:
				continue
			candidate = int(number[:i] + number[i + 1:])
			if candidate > ans:
				ans = candidate
		return str(ans)
