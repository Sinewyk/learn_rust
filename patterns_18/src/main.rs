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
}

fn handle_point_ref(&(x, y): &(i32, i32)) {
	println!("{}, {}", x, y);
	return ();
}

fn handle_point_move((x, y): (i32, i32)) {
	println!("{}, {}", x, y);
	return ();
}
