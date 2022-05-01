def evalKeyStrokes(s: str) -> str:
	s_val = ""
	for c in s:
		if c == "#":
			if len(s_val) > 0:
				s_val = s_val[:-1]
		else:
			s_val += c
	return s_val

class Solution:
	def backspaceCompare(self, s: str, t: str) -> bool:
		s_val = evalKeyStrokes(s)
		t_val = evalKeyStrokes(t)
		return s_val == t_val
