use guessing_mod::guessing::{self, get_random_number, Guess};
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret = get_random_number(0, 100);
    let mut guessing = String::new();

    println!("Secret is {secret}");

    println!("Enter your number(0 => 100): ");
    io::stdin()
        .read_line(&mut guessing)
        .expect("failed to read_line");

    let guessing: i32 = guessing.trim().parse().expect("failed to parse");
    let guess = Guess::new(guessing);

    if let Ordering::Equal = guess.value().cmp(&secret) {
        println!("Correct");
    } else {
        println!("Wrong");
    }
}
