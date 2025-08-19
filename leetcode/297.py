# LeetCode/297 - Serialize and Deserialize Binary Tree

# Definition for a binary tree node.
# class TreeNode(object):
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Codec:

    def serialize(self, root):
        """Encodes a tree to a single string.
        
        :type root: TreeNode
        :rtype: str
        """

        if root is None: return ""

        values = []
        edges = []

        q = [root]
        while len(q) > 0:
            node = q.pop(0)
            values.append(str(node.val))
            i = idxr = len(values) + len(q)
            vall = ""
            valr = ""
            if node.left is not None:
                vall = i
                q.append(node.left)
            if node.right is not None:
                valr = i + 1 if node.left is not None else i
                q.append(node.right)
            edges.append(f"{vall};{valr}")

        values = " ".join(values)
        edges = " ".join(edges)
        
        return f"{values}\n{edges}"
        

    def deserialize(self, data):
        """Decodes your encoded data to tree.
        
        :type data: str
        :rtype: TreeNode
        """

        if data == "": return None

        values, edges = data.split("\n")

        nodes = [TreeNode(int(v)) for v in values.split(" ")]
        
        for i, e in enumerate(edges.split(" ")):
            l, r = e.split(";")
            if l != "": nodes[i].left = nodes[int(l)]
            if r != "": nodes[i].right = nodes[int(r)]
        
        return nodes[0]

class CodecAlt:

    def serialize(self, root):
        """Encodes a tree to a single string.
        
        :type root: TreeNode
        :rtype: str
        """

        if root is None: return ""

        r = str(root.val)
        q = [root.left, root.right]

        while len(q) > 0:
            node = q.pop(0)
            if node is None:
                r += " "
                continue
            else:
                r += " " + str(node.val)
                q.append(node.left)
                q.append(node.right)

        return r
        

    def deserialize(self, data):
        """Decodes your encoded data to tree.
        
        :type data: str
        :rtype: TreeNode
        """
        print(data)
        nodes = [
            None if v == "" else TreeNode(int(v))
            for v in data.split(" ")
        ]

        n = len(nodes)

        nones = 0

        for i in range(n):
            if nodes[i] is None:
                nones += 1
                continue
            j = 2 * (i - nones)
            if j + 2 >= n: continue
            nodes[i].left  = nodes[j + 1]
            nodes[i].right = nodes[j + 2]

        return nodes[0]
        

# Your Codec object will be instantiated and called as such:
# ser = Codec()
# deser = Codec()
# ans = deser.deserialize(ser.serialize(root))
