use std::{fs::File, io::ErrorKind};

fn main() {
    let f = File::open("try.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("try.txt") {
                Ok(file_created) => file_created,
                Err(error) => panic!("Unable to create file: {error:?}"),
            },
            other_error => {
                panic!("Unable to open file: {other_error:?}")
            }
        },
    };
}
