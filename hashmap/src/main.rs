use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("blue"), 0);
    scores.insert(String::from("red"), 0);

    let score = scores.get("blue").copied().unwrap_or(0);
    println!("the blue team's score is {score}");

    for (_, value) in scores {
        println!(" {value}");
    }
}
