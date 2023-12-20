use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret = generate_random_number();
    println!("Secret: {secret}");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Failed to parse");

    match guess.cmp(&secret) {
        Ordering::Less => println!("less"),
        Ordering::Greater => println!("Greater"),
        Ordering::Equal => println!("Equal")
    }
}

fn generate_random_number() -> u32 {
    let num = rand::thread_rng().gen_range(1..=100);
    num
}