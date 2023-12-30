use std::fs;
use std::error::Error;

#[derive(Debug)]
pub struct Config {
    pub file_path: String,
    pub query: String,
}

impl Config {
    pub fn build<T: Iterator<Item = String>>(mut args: T) -> Result<Self, &'static str> {
        args.next();

        let query = match args.next() {
            Some(value) => value,
            None => return Err("Failed to get query")
        };

        let file_path =  match args.next() {
            Some(value) => value,
            None => return Err("Failed to file path")
        };

        Ok(Self {
            query,
            file_path
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|x| x.contains(query))
        .collect()
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let result = search(&config.query, &contents);

    println!("Result:");
    for line in result {
        println!("{line}");
    }

    Ok(())
}