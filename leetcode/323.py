# LeetCode/323 - Number of Connected Components In An Undirected Graph


class Solution:
    def countComponents(self, n: int, edges: list[list[int]]) -> int:
        graph: dict[int, list[int]] = {}
        for i in range(n):
            graph[i] = []
        for a, b in edges:
            graph[a].append(b)
            graph[b].append(a)

        seen = [False] * n
        components = 0

        for i in range(n):
            if seen[i]:
                continue
            components += 1
            seen[i] = True
            stack = [i]
            while len(stack) > 0:
                node = stack.pop()
                for nxt in graph[node]:
                    if seen[nxt]:
                        continue
                    seen[nxt] = True
                    stack.append(nxt)

        return components
