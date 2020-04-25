#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32,
}

impl Rectangle {
	fn can_hold(&self, other: &Rectangle) -> bool {
		self.width > other.width && self.height > other.height
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn exploration() {
		assert_eq!(2 + 2, 4);
	}

	#[test]
	fn it_panics() {
		panic!("Make this test fail!");
	}
}
