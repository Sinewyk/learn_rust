use oop_17::{Button, Draw, Post, Screen};

struct SelectBox {
	width: u32,
	height: u32,
	options: Vec<String>,
}

impl Draw for SelectBox {
	fn draw(&self) {
		// code to actually draw a select box
	}
}

fn main() {
	let screen = Screen {
		components: vec![
			Box::new(SelectBox {
				width: 75,
				height: 10,
				options: vec![
					String::from("Yes"),
					String::from("Maybe"),
					String::from("No"),
				],
			}),
			Box::new(Button {
				width: 50,
				height: 10,
				label: String::from("OK"),
			}),
		],
	};

	screen.run();

	// This doesn't work because String::from("test") doesn't implement oop_17::Draw
	/* let screen = Screen {
		components: vec![Box::new(String::from("test"))],
	}; */

	let mut draft = Post::new();

	draft.add_text("I ate a salad for lunch today");

	let post_in_review = draft.request_review();

	let post = post_in_review.approve();

	assert_eq!("I ate a salad for lunch today", post.content());
}
