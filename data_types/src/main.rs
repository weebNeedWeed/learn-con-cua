fn main() {
    let v = get_value();
    println!("Value after loop: {v}");
}

fn get_value() -> i32 {
    let mut count = 0;

    for _elm in 1..4 {
        count += 1;
    }

    count
}
