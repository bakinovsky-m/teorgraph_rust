pub struct Queue {
	array: Vec<i32>
}

impl Queue {
	pub fn new() -> Queue{
		Queue{
			array: Vec::new(),
		}
	}
	
	pub fn push(& mut self, value: i32) {
		self.array.push(value);
	}

	pub fn pop(& mut self) -> Option<i32> {
		self.array.reverse();
		let ret = self.array.pop();
		self.array.reverse();
		return ret;
	}

	pub fn len(& self) -> usize {
		return self.array.len();
	}
}