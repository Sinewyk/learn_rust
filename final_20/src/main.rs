use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
	let body = fs::read_to_string("hello.html").unwrap();
	let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

	for stream in listener.incoming() {
		let stream = stream.unwrap();

		handle_connection(stream, &body);
	}
}

fn handle_connection(mut stream: TcpStream, body: &str) {
	let mut buffer = [0; 1024];

	stream.read(&mut buffer).unwrap();

	// buffer is [u8; _] ... should be something about the size of it
	// and buffer[..] is [u8], should also be something about the size of it, the buffer itself
	// knows how much it's been filled when writing into it ?
	// println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

	let full_response = format!(
		"HTTP/1.1 OK\r\nContent-Length: {}\r\n\r\n{}",
		body.len(),
		body
	);

	stream.write(full_response.as_bytes()).unwrap();
	stream.flush().unwrap();
}
