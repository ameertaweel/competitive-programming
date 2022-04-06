# Definition for singly-linked list.
# class ListNode:
#	 def __init__(self, val=0, next=None):
#		 self.val = val
#		 self.next = next
class Solution:
	def swapNodes(self, head: Optional[ListNode], k: int) -> Optional[ListNode]:
		begPrev = None
		beg = head
		
		# Find the kth node from the beginning
		for i in range(1, k):
			begPrev = beg
			beg = beg.next
		
		ptr = beg
		endPrev = None
		end = head
		
		# Find the kth node from the end
		# Using the kth node from the beginning
		while ptr.next != None:
			ptr = ptr.next
			endPrev = end
			end = end.next
		
		if endPrev == beg:
			# From:
			# begPrev -> beg -> end
			# To:
			# begPrev -> end -> beg
			beg.next = end.next
			end.next = beg
		elif begPrev == end:
			# From:
			# endPrev -> end -> beg
			# To:
			# endPrev -> beg -> end
			end.next = beg.next
			beg.next = end
		else:
			# From:
			# begPrev -> beg -> endPrev -> end
			# To:
			# begPrev -> end -> endPrev -> beg
			beg.next, end.next = end.next, beg.next

		if begPrev != None:
			if begPrev != end:
				begPrev.next = end
		else:
			head = end
		if endPrev != None:
			if endPrev != beg:
				endPrev.next = beg
		else:
			head = beg
			
		return head
