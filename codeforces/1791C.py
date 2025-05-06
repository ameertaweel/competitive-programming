# Link To Problem:
# https://codeforces.com/problemset/problem/1791/C

t = int(input())

for _ in range(t):
	n = int(input())
	s = input()

	beg = 0
	end = n - 1

	while end > beg and s[beg] != s[end]:
		beg += 1
		end -= 1

	print(end - beg + 1)
