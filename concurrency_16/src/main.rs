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

	// handle.join().unwrap();
}
