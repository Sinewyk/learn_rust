pub trait Draw {
	fn draw(&self);
}

pub struct Screen {
	pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
	pub fn run(&self) {
		for component in self.components.iter() {
			component.draw();
		}
	}
}

pub struct Button {
	pub width: u32,
	pub height: u32,
	pub label: String,
}

impl Draw for Button {
	fn draw(&self) {}
}

pub struct Post {
	content: String,
}

pub struct DraftPost {
	content: String,
}
pub struct PendingReviewPost {
	content: String,
}

impl Post {
	pub fn new() -> DraftPost {
		DraftPost {
			content: String::new(),
		}
	}

	pub fn content(&self) -> &str {
		&self.content
	}
}

impl DraftPost {
	pub fn add_text(&mut self, text: &str) {
		self.content.push_str(text);
	}

	pub fn request_review(self) -> PendingReviewPost {
		PendingReviewPost {
			content: self.content,
		}
	}
}

impl PendingReviewPost {
	pub fn approve(self) -> Post {
		Post {
			content: self.content,
		}
	}
}
