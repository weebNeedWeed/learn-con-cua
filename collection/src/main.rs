use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter your string: ");
    io::stdin().read_line(&mut input).expect("Cannot read line");

    loop {
        println!("enter index which used to get char at: ");
        let mut index = String::new();
        match io::stdin().read_line(&mut index) {
            Ok(_) => (),
            Err(_) => continue,
        }

        let index: usize = match index.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if index >= input.len() {
            continue;
        }

        let mut chars = input.chars();
        let c: char = match chars.nth(index) {
            Some(c) => c,
            None => continue,
        };

        println!("this char is {c}");
        break;
    }
}
