class Solution:
	def shiftGrid(self, grid: List[List[int]], k: int) -> List[List[int]]:
		columns = list(map(list, zip(*grid)))

		for i in range(k):
			columns = [columns[-1]] + columns[:-1]
			columns[0] = [columns[0][-1]] + columns[0][:-1]

		return list(map(list, zip(*columns)))
