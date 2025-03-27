# LeetCode/261 - Graph Valid Tree


class Solution:
    def validTree(self, n: int, edges: list[list[int]]) -> bool:
        graph: dict[int, set[int]] = {}

        for i in range(n):
            graph[i] = set()

        for a, b in edges:
            graph[a].add(b)
            graph[b].add(a)

        # Node, Parent
        stack = [(0, -1)]

        seen = [False] * n
        seen[0] = True

        while len(stack) > 0:
            node, parent = stack.pop()

            for neighbor in graph[node]:
                if neighbor == parent:
                    continue
                if seen[neighbor]:
                    return False
                seen[neighbor] = True
                stack.append((neighbor, node))

        return all(seen)
