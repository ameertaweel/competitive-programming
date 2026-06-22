# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def removeNthFromEnd(self, head: Optional[ListNode], n: int) -> Optional[ListNode]:
        # if head.next is None: return None
        # if head.next.next is None:
        #     if n == 2: return head.next
        #     head.next = None
        #     return head

        frnt = head
        back = head
        diff = 0

        while frnt.next is not None:
            frnt = frnt.next
            diff = diff + 1
            if diff > n:
                back = back.next
                diff = diff - 1

        if diff < n: return head.next

        back.next = back.next.next
        return head
