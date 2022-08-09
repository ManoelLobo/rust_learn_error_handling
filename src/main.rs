use std::fs::File;

fn main() {
    let f = File::open("try.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("at the file! {error:?}"),
    };
}
