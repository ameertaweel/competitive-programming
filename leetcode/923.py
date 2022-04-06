from collections import Counter
import math

class Solution:
	def threeSumMulti(self, arr: List[int], target: int) -> int:
		n = len(arr)
		c = Counter(arr)

		x = min(target, 100)
		y = min(target - x, 100)
		z = target - x - y
		ans = 0
		mod = 10 ** 9 + 7

		while x >= target / 3:
			while y >= (target - x) / 2:
				z = target - x - y

				if x == y and y == z:
					ans += math.comb(c.get(x, 0), 3)
				elif x == y:
					ans += math.comb(c.get(x, 0), 2) * c.get(z, 0)
				elif y == z:
					ans += c.get(x, 0) * math.comb(c.get(y, 0), 2)
				else:
					ans += c.get(x, 0) * c.get(y, 0) * c.get(z, 0)
				ans = ans % mod

				y -= 1
			x -= 1
			y = min(target - x, x)

		return ans
