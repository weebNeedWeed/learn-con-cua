#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn size(&self) -> i32 {
        self.width * self.height
    }

    fn new(width: i32, height: i32) -> Self {
        Self {
            width: width,
            height: height,
        }
    }
}

fn main() {
    let rec = Rectangle::new(50, 50);

    println!("{rec:?}");

    println!("Size of this rec is: {}", rec.size());
}
