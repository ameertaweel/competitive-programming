class Solution:
	def removeDuplicates(self, s: str, k: int) -> str:
		freq = []
		last_char = None
		last_freq = None
		
		for c in s:
			if c != last_char:
				freq.append([last_char, last_freq])
				last_char = c
				last_freq = 1
			else:
				last_freq += 1
				if last_freq == k:
					last_char, last_freq = freq.pop()
		
		freq.append([last_char, last_freq])
		freq = freq[1:]
		
		return "".join([f[0] * f[1] for f in freq])
