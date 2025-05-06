# Link To Problem:
# https://codeforces.com/problemset/problem/271/A

y = int(input())

# The output year should be strictly larger than the input year
y += 1

while len(set(list(str(y)))) != len(str(y)):
	y += 1

print(y)
