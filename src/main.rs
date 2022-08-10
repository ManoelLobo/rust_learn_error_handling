use std::{
    fs::File,
    io::{self, Read},
};

fn main() {
    let name = read_username_from_file().unwrap();
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("try.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
