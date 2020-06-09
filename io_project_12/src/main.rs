use io_project_12::Config;
use std::env;
use std::process;

fn main() {
	let args: Vec<String> = env::args().collect();

	let config = match Config::new(&args) {
		Ok(c) => c,
		Err(err) => {
			println!("Problem parsing arguments: {}", err);
			process::exit(1);
		}
	};

	println!("Searching for {}", config.query);
	println!("In file {}", config.filename);

	if let Err(e) = io_project_12::run(config) {
		println!("Application error: {}", e);

		process::exit(1);
	};
}
