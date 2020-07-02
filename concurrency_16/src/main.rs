use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
	let some_variable = vec![1, 2, 3];

	let handle = thread::spawn(move || {
		println!("{:?}", &some_variable);
	});

	handle.join().unwrap();

	for i in 1..5 {
		println!("hi number {} from the main thread", i);
		thread::sleep(Duration::from_millis(1));
	}

	let (tx, rx) = mpsc::channel();

	thread::spawn(move || {
		let vals = vec![
			String::from("hi"),
			String::from("from"),
			String::from("the"),
			String::from("thread"),
		];
		for val in vals {
			tx.send(val).unwrap();
			thread::sleep(Duration::from_millis(100));
		}
		// println!("{}", val); // <= you can't do that
	});

	for received in rx {
		println!("Got: {}", received);
	}

	// handle.join().unwrap();
}
