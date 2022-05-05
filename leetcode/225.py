class MyStack:

	def __init__(self):
		self.q0 = []
		self.q1 = []
		self.qi = 0
		self.t = None

	def push(self, x: int) -> None:
		# Determine which queue has the data
		q = self.q0 if self.qi == 0 else self.q1

		q.append(x)
		self.t = x
		

	def pop(self) -> int:
		# Determine which queue has the data, and which queue will have the data
		qa, qb = (self.q0, self.q1) if self.qi == 0 else (self.q1, self.q0)
		
		while len(qa) > 1:
			e = qa.pop(0)
			qb.append(e)
			self.t = e
		
		self.qi = 0 if self.qi == 1 else 1
		return qa.pop(0)


	def top(self) -> int:
		return self.t
		

	def empty(self) -> bool:
		# Determine which queue has the data
		q = self.q0 if self.qi == 0 else self.q1
		
		return len(q) == 0
		


# Your MyStack object will be instantiated and called as such:
# obj = MyStack()
# obj.push(x)
# param_2 = obj.pop()
# param_3 = obj.top()
# param_4 = obj.empty()
