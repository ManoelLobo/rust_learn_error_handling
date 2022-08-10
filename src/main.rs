use std::{
    fs::File,
    io::{self, Read},
};

fn main() {
    let name = read_username_from_file().unwrap();
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("try.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
