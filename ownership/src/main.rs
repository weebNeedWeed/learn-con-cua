fn main() {
    let s = String::from("Hello world");

    let first_word = find_first_word_using_slice(&s);

    println!("First space is: {first_word}");
}

#[allow(dead_code)]
fn find_first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &word) in bytes.iter().enumerate() {
        if word == b' ' {
            return i;
        }
    }

    s.len()
}

fn find_first_word_using_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &word) in bytes.iter().enumerate() {
        if word == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
