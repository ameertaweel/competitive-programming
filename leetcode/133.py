# LeetCode/133 - Clone Graph

from __future__ import annotations


class Node:
    def __init__(self, val: int = 0, neighbors: list[Node] | None = None):
        self.val: int = val
        self.neighbors: list[Node] = neighbors if neighbors is not None else []


class Solution:
    def cloneGraph(self, node: Node | None) -> Node | None:
        if node == None:
            return None

        new_nodes = {node.val: Node(node.val)}
        stack = [node]

        while len(stack) > 0:
            old_node = stack.pop()
            new_node = new_nodes[old_node.val]

            for n in old_node.neighbors:
                if n.val not in new_nodes:
                    new_nodes[n.val] = Node(n.val)
                    stack.append(n)
                new_node.neighbors.append(new_nodes[n.val])

        return new_nodes[node.val]
