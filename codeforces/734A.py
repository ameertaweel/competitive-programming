# Link To Problem:
# https://codeforces.com/problemset/problem/734/A

n = int(input())
s = input()

score_anton = 0
score_danik = 0

for char in s:
	if char == "A":
		score_anton += 1
	else:
		score_danik += 1

if   score_anton > score_danik:
	print("Anton")
elif score_danik > score_anton:
	print("Danik")
else:
	print("Friendship")
