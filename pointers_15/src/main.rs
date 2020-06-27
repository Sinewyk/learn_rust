use std::ops::Deref;

pub enum List {
	Cons(i32, Box<List>),
	Nil,
}

// use crate::List::{Cons, Nil};

fn main() {
	// let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

	let x = 5;
	let y = &x;

	assert_eq!(5, x);
	assert_eq!(5, *y);

	let x = 5;
	let y = Box::new(x);

	assert_eq!(5, x);
	assert_eq!(5, *y);

	let x = 5;
	let y = MyBox::new(5);

	assert_eq!(5, x);
	assert_eq!(5, *y);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
	fn new(x: T) -> MyBox<T> {
		MyBox(x)
	}
}

impl<T> Deref for MyBox<T> {
	type Target = T;

	fn deref(&self) -> &T {
		&self.0
	}
}
