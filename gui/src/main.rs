use gui::{Draw, Screen};

struct TextBox {
    width: u32,
    height: u32,
}

impl Draw for TextBox {
    fn draw(&self) {
        println!("{} {}", self.width, self.height)
    }
}

struct Button {
    width: u32,
    height: u32,
    clicked: bool,
}

impl Draw for Button {
    fn draw(&self) {
        println!("{} {} {}", self.width, self.height, self.clicked)
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(TextBox {
                width: 1,
                height: 2,
            }),
            Box::new(Button {
                width: 1,
                height: 0,
                clicked: true,
            }),
        ],
    };

    screen.run()
}
