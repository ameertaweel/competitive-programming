# Link To Problem:
# https://codeforces.com/problemset/problem/677/A

n, h = tuple(int(i) for i in input().split(" "))
a = tuple(int(i) for i in input().split(" "))

# Solution 1:
# min_width = 0
# for ai in a:
# 	if ai <= h:
# 		min_width += 1
# 	else:
# 		min_width += 2
# print(min_width)

# Solution 2:
min_width = n
for ai in a:
	if ai > h: min_width += 1
print(min_width)
