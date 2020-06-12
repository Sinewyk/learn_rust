use std::error::Error;
use std::fs;

pub struct Config {
	pub query: String,
	pub filename: String,
}

impl Config {
	pub fn new(args: &[String]) -> Result<Config, &'static str> {
		if args.len() < 3 {
			return Err("Not enough arguments, need 2 for query and filename");
		}
		let query = args[1].clone();
		let filename = args[2].clone();

		Ok(Config { query, filename })
	}
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
	let contents = fs::read_to_string(config.filename)?;

	let res = search(&config.query, &contents);

	for match_ in res {
		println!("{}", match_);
	}

	Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	let mut res = vec![];
	for line in contents.lines() {
		if line.contains(query) {
			res.push(line);
		}
	}
	res
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn one_result() {
		let query = "duct";
		let contents = "\
Rust:
safe, fast, productive.
Pick three.";

		assert_eq!(vec!["safe, fast, productive."], search(query, contents));
	}
}