use std::marker::PhantomData;
use std::thread;
use std::time::Duration;

fn main() {
	let simulated_user_specific_value = 10;
	let simulated_random_number = 7;

	generate_workout(simulated_user_specific_value, simulated_random_number);
}

fn generate_workout(intensity: u32, random_number: u32) {
	let mut expensive_result = Cacher::new(|num| {
		println!("Calculating slowly ...");
		thread::sleep(Duration::from_micros(2));
		num
	});

	if intensity < 25 {
		println!("Today, do {} pushups!", expensive_result.value(intensity));
		println!("Next, do {} situps", expensive_result.value(intensity));
	} else {
		if random_number == 3 {
			println!("Take a break today! Remember to stay hydrated!");
		} else {
			println!(
				"Today, run for {} minutes!",
				expensive_result.value(intensity)
			);
		}
	}
}

struct Cacher<T, U, V>
where
	T: Fn(U) -> V,
	V: Copy,
{
	calculation: T,
	value: Option<V>,
	phantom: PhantomData<U>,
}

impl<T, U, V> Cacher<T, U, V>
where
	T: Fn(U) -> V,
	V: Copy,
{
	fn new(calculation: T) -> Cacher<T, U, V> {
		Cacher {
			calculation,
			value: None,
			phantom: PhantomData,
		}
	}

	fn value(&mut self, arg: U) -> V {
		match self.value {
			Some(v) => v,
			None => {
				let v = (self.calculation)(arg);
				self.value = Some(v);
				v
			}
		}
	}
}
