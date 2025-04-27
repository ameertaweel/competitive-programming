# DMOPC '18 September Contest - Problem 2 - Sorting

n, k = [int(i) for i in input().split(" ")]
p = [int(i) for i in input().split(" ")]

valid = True

for i in range(n):
    track = (i % k) + 1
    if (p[i] - track) % k != 0:
        valid = False
        break

print("YES" if valid else "NO")
