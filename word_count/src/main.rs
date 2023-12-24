use std::collections::HashMap;
use std::io::stdin;

fn main() {
    let mut count = HashMap::new();

    let mut str = String::new();
    stdin().read_line(&mut str).expect("cannot read_line");

    for word in str.split_whitespace() {
        let entry = count.entry(word).or_insert(0);
        *entry += 1;
    }

    println!("{count:#?}");
}
