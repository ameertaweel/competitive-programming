from collections import defaultdict

class Solution:
	def calcEquation(self, equations: List[List[str]], values: List[float], queries: List[List[str]]) -> List[float]:
		graph = defaultdict(lambda : [])
		answers = []
		
		for (a, b), val in zip(equations, values):
			if len(graph[a]) == 0:
				graph[a].append((a, 0 if val == 0 else 1))
			graph[a].append((b, val))
			graph[b].append((b, 1))
			graph[b].append((a, 1 / val))
		
		for a, b in queries:
			visited = defaultdict(lambda : False)
			if len(graph[a]) == 0:
				answers.append(-1)
				continue
			stack = [(a, graph[a][0][1])]
			ans = -1
			while len(stack) > 0:
				node, val = stack.pop()
				if visited[node]:
					continue
				visited[node] = True
				if node == b:
					ans = val
					break
				for neighbor, edge_val in graph[node]:
					stack.append((neighbor, val * edge_val))

			answers.append(ans)
		
		return answers
