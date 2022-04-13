class Solution:
	def generateMatrix(self, n: int) -> List[List[int]]:
		M = [[0] * n for i in range(n)]
		
		dirs = ((0, 1), (1, 0), (0, -1), (-1, 0))
		
		i = j = d = 0;
		
		for c in range(n ** 2):
			M[i][j] = c + 1
			ni = i + dirs[d][0]
			nj = j + dirs[d][1]
			if ni >= 0 and ni < n and nj >= 0 and nj < n and M[ni][nj] == 0:
				i = ni
				j = nj
			else:
				d = (d + 1) % 4
				i += dirs[d][0]
				j += dirs[d][1]
		
		return M
