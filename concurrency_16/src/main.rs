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
		let val = String::from("hi");
		tx.send(val).unwrap();
	});

	let received = rx.recv().unwrap();

	println!("Got: {}", received);

	// handle.join().unwrap();
}
