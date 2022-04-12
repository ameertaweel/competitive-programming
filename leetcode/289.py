class Solution:
	def gameOfLife(self, board: List[List[int]]) -> None:
		m = len(board)
		n = len(board[0])
		
		neighbors = [(0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1), (-1, 0), (-1, 1)]
		for i in range(m):
			for j in range(n):
				alive = 0
				for neighbor in neighbors:
					ni = i + neighbor[0]
					nj = j + neighbor[1]
					alive += 1 if (ni >= 0 and ni < m and nj >= 0 and nj < n and board[ni][nj] > 0) else 0
				if board[i][j] == 1:
					if alive != 2 and alive != 3:
						board[i][j] = 2
				elif alive == 3:
					board[i][j] = -1

		for i in range(m):
			for j in range(n):
				if board[i][j] == 2:
					board[i][j] = 0
				elif board[i][j] == -1:
					board[i][j] = 1
