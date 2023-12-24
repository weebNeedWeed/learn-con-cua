pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl Point<f32> {
    pub fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl Point<f32> {
    fn show(&self) {
        println!("x = {}, y = {}", self.x, self.y);
    }
}
