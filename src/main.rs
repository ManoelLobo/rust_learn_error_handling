use std::{fs, io};

fn main() {
    let name = read_username_from_file().unwrap();
}

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("try.txt")
}
