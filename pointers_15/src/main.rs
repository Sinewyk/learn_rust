use std::ops::Deref;

use crate::List::{Cons, Nil};
use std::rc::Rc;

#[derive(Debug)]
enum List2 {
	Cons2(Rc<RefCell<i32>>, Rc<List2>),
	Nil2,
}

use crate::List2::{Cons2, Nil2};
use std::cell::RefCell;

/* enum List {
	Cons(i32, Box<List>),
	Nil,
} */

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

	let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
	println!("count after creating a = {}", Rc::strong_count(&a));
	let b = Cons(3, Rc::clone(&a));
	println!("count after creating b = {}", Rc::strong_count(&a));
	{
		let c = Cons(4, Rc::clone(&a));
		println!("count after creating c = {}", Rc::strong_count(&a));
	}
	println!("count after c goes out of scope = {}", Rc::strong_count(&a));

	let value = Rc::new(RefCell::new(5));

	let a = Rc::new(Cons2(Rc::clone(&value), Rc::new(Nil2)));

	let b = Cons2(Rc::new(RefCell::new(6)), Rc::clone(&a));
	let c = Cons2(Rc::new(RefCell::new(10)), Rc::clone(&a));

	*value.borrow_mut() += 10;

	println!("a after = {:?}", a);
	println!("b after = {:?}", b);
	println!("c after = {:?}", c);
}

enum List {
	Cons(i32, Rc<List>),
	Nil,
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
