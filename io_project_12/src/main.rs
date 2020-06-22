use io_project_12::Config;
use std::env;
use std::process;

fn main() {
	let config = match Config::new(env::args()) {
		Ok(c) => c,
		Err(err) => {
			eprintln!("Problem parsing arguments: {}", err);
			process::exit(1);
		}
	};

	if let Err(e) = io_project_12::run(config) {
		eprintln!("Application error: {}", e);

		process::exit(1);
	};
}
