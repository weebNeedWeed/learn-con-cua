use ch10::dot1_generics::Point;
use ch10::dot2::{NewsArticle, notify, Summary};

fn main() {
    let a = NewsArticle {
        headline: String::from("haha"),
        location: String::from("location"),
        author: String::from("author"),
        content: String::from("Content"),
    };

    println!("Summary = {}", a.summarize());

    let p = Point { x: 1.2, y: 1.2 };

    println!("distance_from_origin = {}", p.distance_from_origin());

    notify(&2);
}
