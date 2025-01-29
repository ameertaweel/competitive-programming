class Solution:

	def encode(self, strs: List[str]) -> str:
		res = ""
		for s in strs:
			res += ";"
			for c in s:
				if c == ",": res += ",,"
				elif c == ";": res += ",;"
				else: res += c
		return res

	def decode(self, s: str) -> List[str]:
		res = []
		i = 0
		while i < len(s):
			if s[i] == ",":
				res[-1] += s[i + 1]
				i += 1
			elif s[i] == ";":
				res.append("")
			else:
				res[-1] += s[i]
			i += 1
		return res
