use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret: u32 = rand::thread_rng().gen_range(1..=100);
    println!("Secret num = {secret}");

    loop {
        println!("Please enter your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret) {
            Ordering::Less => println!("Less"),
            Ordering::Greater => println!("Greater"),
            Ordering::Equal => {
                println!("Equal!");
                break;
            }
        }
    }
}
