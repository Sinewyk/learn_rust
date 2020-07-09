#[allow(unused_variables)]
fn main() {
	let x = Some(1);

	// match is exhaustive
	let x = match x {
		Some(x) => x,
		None => 1,
	};

	// pattern non refutable
	let x = 1;

	let y: Option<&str> = None;

	// You can't do that, that's refutable
	// let Some(y) = y;

	// If you wanna do that, use if let
	if let Some(y) = y {
		println!("{}", y);
	}
	// which is non-exhaustive, notice how the compiler didn't whine about missing None branch

	let mut stack = Vec::new();

	stack.push(1);
	stack.push(2);
	stack.push(3);

	for x in &stack {
		println!("{}", *x);
	}

	while let Some(top) = stack.pop() {
		println!("{}", top);
	}

	let v = vec!['a', 'b', 'c'];

	for (index, value) in v.iter().enumerate() {
		println!("{} is at index {}", value, index);
	}

	// some destructuring
	let (x, y, _) = (1, 2, 3);

	let point = (1, 2);

	// @todo: figure out why not borrow after move
	// but I suspect it was copied
	handle_point_move(point);
	handle_point_ref(&point);

	let x = 1;

	match x {
		1 | 2 => println!("one or two"),
		3 => println!("three"),
		_ => println!("anything"),
	}

	let x = 2;

	match x {
		// 1..3 => println!("one to three excluded"), // exclusive is experimental
		1..=3 => println!("one to three"),
		_ => println!("anything"),
	}

	let p = Point { x: 0, y: 0 };

	match p {
		Point { x: 0, y: 0 } => println!("On the origin of the axis at (0, 0)"),
		Point { x, y: 0 } => println!("On the x axis at {}", x),
		Point { x: 0, y } => println!("On the y axis at {}", y),
		Point { x, y } => println!("On neither axis: ({}, {})", x, y),
	}

	let msg = Message::ChangeColor(0, 160, 255);

	match msg {
		Message::Quit => println!("The Quit variant has no data to destructure."),
		Message::Move { x, y } => {
			println!("Move in the x direction {} and in the y direction {}", x, y);
		}
		Message::Write(text) => println!("Text message: {}", text),
		Message::ChangeColor(r, g, b) => {
			println!("Change the color to red {}, green {}, and blue {}", r, g, b)
		}
	}
}

fn handle_point_ref(&(x, y): &(i32, i32)) {
	println!("{}, {}", x, y);
	return ();
}

fn handle_point_move((x, y): (i32, i32)) {
	println!("{}, {}", x, y);
	return ();
}

#[allow(dead_code)]
enum Message {
	Quit,
	Move { x: i32, y: i32 },
	Write(String),
	ChangeColor(i32, i32, i32),
}

struct Point {
	x: i32,
	y: i32,
}
