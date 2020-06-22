use std::collections::hash_map::HashMap;
use std::hash::Hash;
use std::thread;
use std::time::Duration;

fn main() {
	let simulated_user_specific_value = 10;
	let simulated_random_number = 7;

	generate_workout(simulated_user_specific_value, simulated_random_number);

	let mut c = Cacher::new(|a| a);

	let v1 = c.value(1);
	let v2 = c.value(2);

	assert_ne!(v1, v2);

	let x = 4;

	let equal_to_x = |z| z == x;

	let y = 4;

	assert!(equal_to_x(y));

	let v1: Vec<i32> = vec![1, 2, 3];

	let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
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
	U: Eq + Hash + Copy,
	V: Copy,
{
	calculation: T,
	map: HashMap<U, V>,
}

impl<T, U, V> Cacher<T, U, V>
where
	T: Fn(U) -> V,
	U: Eq + Hash + Copy,
	V: Copy,
{
	fn new(calculation: T) -> Cacher<T, U, V> {
		Cacher {
			calculation,
			map: HashMap::new(),
		}
	}

	fn value(&mut self, arg: U) -> V {
		match self.map.get(&arg) {
			Some(v) => *v,
			None => {
				let v = (self.calculation)(arg);
				self.map.insert(arg, v);
				v
			}
		}
	}
}

#[derive(PartialEq, Debug)]
struct Shoe {
	size: u32,
	style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
	shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn filters_by_size() {
		let shoes = vec![
			Shoe {
				size: 10,
				style: String::from("sneaker"),
			},
			Shoe {
				size: 13,
				style: String::from("sandal"),
			},
			Shoe {
				size: 10,
				style: String::from("boot"),
			},
		];

		let in_my_size = shoes_in_my_size(shoes, 10);

		assert_eq!(
			in_my_size,
			vec![
				Shoe {
					size: 10,
					style: String::from("sneaker")
				},
				Shoe {
					size: 10,
					style: String::from("boot")
				},
			]
		);
	}
}

struct Counter {
	count: u32,
}

impl Counter {
	fn new() -> Counter {
		Counter { count: 0 }
	}
}

impl Iterator for Counter {
	type Item = u32;

	fn next(&mut self) -> Option<Self::Item> {
		if self.count < 5 {
			self.count += 1;
			Some(self.count)
		} else {
			None
		}
	}
}

#[test]
fn calling_next_directly() {
	let mut counter = Counter::new();

	assert_eq!(counter.next(), Some(1));
	assert_eq!(counter.next(), Some(2));
	assert_eq!(counter.next(), Some(3));
	assert_eq!(counter.next(), Some(4));
	assert_eq!(counter.next(), Some(5));
	assert_eq!(counter.next(), None);
}
