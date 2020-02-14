use std::fs::{self, File};
use std::io::{Error, ErrorKind, Read};

fn main() {
    match File::open("hello.txt") {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    cleanup();

    File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    match read_username_from_file() {
        Ok(s) => println!(
            "stuff in hello.txt: {}",
            if s == "" { "nothing inside :'(" } else { &s }
        ),
        Err(e) => panic!("error reading from file :'(, {:?}", e),
    };

    cleanup();
}

fn read_username_from_file() -> Result<String, Error> {
    fs::read_to_string("hello.txt")
}

fn cleanup() {
    std::fs::remove_file("hello.txt").expect("Could not remove hello.txt");
}
