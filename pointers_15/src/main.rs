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

	let name = MyBox::new(String::from("Rust"));
	hello(&name);

	let c = CustomSmartPointer {
		data: String::from("my stuff"),
	};
	println!("created");
	drop(c);
	println!("eom");
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

fn hello(name: &str) {
	println!("Hello, {}", name);
}

struct CustomSmartPointer {
	data: String,
}

impl Drop for CustomSmartPointer {
	fn drop(&mut self) {
		println!("Dropping CustomSmartPointer with data '{}'", self.data)
	}
}
