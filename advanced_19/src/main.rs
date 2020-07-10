// Maybe I need to go search how this works
extern "C" {
	fn abs(input: i32) -> i32;
}

fn main() {
	unsafe {
		println!("Absolute value of -3 according to C: {}", abs(-3));
	}
}
