use std::{cell::RefCell, rc::Rc};

enum List {
    Cons(RefCell<i32>, Rc<List>),
    Nil,
}

fn main() {
    let a = Rc::new(List::Cons(RefCell::new(3), Rc::new(List::Nil)));
    let b = List::Cons(RefCell::new(2), Rc::clone(&a));

    let mut i = &b;
    while let List::Cons(v, next) = i {
        *v.borrow_mut() += 10;
        i = next;
        println!("{}", *v.borrow());
    }
}
