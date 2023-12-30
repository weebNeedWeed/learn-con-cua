use std::{ops::Deref, rc::Rc};

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> Self {
        Self(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {name}");
}

fn main() {
    let x = Rc::new(List::Cons(1, Rc::new(List::Nil)));
    let y = Rc::clone(&x);

    if let List::Cons(v, h) = &y as &List {}
}
