# LeetCode/1584 - Min Cost to Connect All Points

import math


class Solution:
    # Use Optimized Prim's Algorithm
    # Finds a minimum spanning tree (MST) of a graph
    # Time Complexity: O(N^2)
    # Space Complexity: O(N)
    def minCostConnectPoints(self, points: list[list[int]]) -> int:
        n = len(points)

        weights = [math.inf] * n
        weights[0] = 0

        in_mst = [False] * n

        edges_used = 0
        cost = 0

        while edges_used < n:
            curr_edge = math.inf
            curr_node = -1

            for node in range(n):
                if (not in_mst[node]) and weights[node] < curr_edge:
                    curr_edge = weights[node]
                    curr_node = node

            edges_used += 1
            cost += int(curr_edge)
            in_mst[curr_node] = True

            for node in range(n):
                if in_mst[node]:
                    continue
                xi, yi = points[curr_node]
                xj, yj = points[node]
                new_weight = abs(xi - xj) + abs(yi - yj)
                if new_weight < weights[node]:
                    weights[node] = new_weight

        return cost
