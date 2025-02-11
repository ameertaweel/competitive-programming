# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def levelOrder(self, root: Optional[TreeNode]) -> List[List[int]]:
        if root == None:
            return []

        queue_curr = [root]
        queue_next = []
        level_curr = []
        ans = []

        while len(queue_curr) > 0:
            for node in queue_curr:
                level_curr.append(node.val)
                if node.left != None:
                    queue_next.append(node.left)
                if node.right != None:
                    queue_next.append(node.right)
            ans.append(level_curr)
            level_curr = []
            queue_curr = queue_next
            queue_next = []

        return ans
