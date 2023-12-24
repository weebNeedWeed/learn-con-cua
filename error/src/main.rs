use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let file_name = "hello.txt";

    let file = match File::open(file_name) {
        Ok(f) => f,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create(file_name) {
                Ok(f) => f,
                Err(err) => panic!("Error occured: {err}"),
            },
            others => panic!("Error occured: {others}"),
        },
    };
}
