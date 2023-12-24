use std::{
    fs::{self, File},
    io::{Error, Read},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let name = "hello.txt";
    let username = read_username_shortest(name)?;
    let last_char = read_last_char(&username).expect("cannot get last char");

    println!("last char = {last_char}");

    Ok(())
}

fn read_username_from_file(file_name: &str) -> Result<String, Error> {
    let mut file = match File::open(file_name) {
        Ok(f) => f,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_shorter(file_name: &str) -> Result<String, Error> {
    let mut file = File::open(file_name)?;
    let mut username = String::new();
    file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_even_shorter(file_name: &str) -> Result<String, Error> {
    let mut username = String::new();
    File::open(file_name)?.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_shortest(file_name: &str) -> Result<String, Error> {
    fs::read_to_string(file_name)
}

fn read_last_char(input: &str) -> Option<char> {
    input.lines().next()?.chars().last()
}
