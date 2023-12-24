use rand::Rng;

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn value(&self) -> i32 {
        self.value
    }

    pub fn new(value: i32) -> Self {
        if value < 0 || value > 100 {
            panic!("Value must be greater than or equal 0 and less than or equal 100");
        }

        Self { value: value }
    }
}

pub fn get_random_number(start: i32, end: i32) -> i32 {
    if start > end {
        panic!("Start number must be lese than end number");
    }

    rand::thread_rng().gen_range(start..=end)
}
