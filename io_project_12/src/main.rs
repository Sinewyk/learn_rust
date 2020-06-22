use io_project_12::Config;
use std::env;
use std::process;

fn main() {
	let config = Config::new(env::args()).unwrap_or_else(|err| {
		eprintln!("Problem parsing arguments: {}", err);
		process::exit(1);
	});

	if let Err(e) = io_project_12::run(config) {
		eprintln!("Application error: {}", e);

		process::exit(1);
	};
}
