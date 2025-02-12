# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def rightSideView(self, root: Optional[TreeNode]) -> List[int]:
        view = []
        # (Node, Level)
        backtrack = [(None, 0)]
        curr_node = root
        curr_level = 0

        while curr_node != None:
            if curr_level == len(view):
                view.append(curr_node.val)

            if curr_node.right == None and curr_node.left == None:
                curr_node, curr_level = backtrack.pop()
                backtrack_node = None
                backtrack_level = 0
            elif curr_node.right == None:
                curr_node = curr_node.left
                curr_level += 1
            else:
                if curr_node.left != None:
                    backtrack.append((curr_node.left, curr_level + 1))
                curr_node = curr_node.right
                curr_level += 1

        return view
