use std::{sync::Mutex, thread};
use std::sync::Arc;

fn main() {
    let m = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let m = Arc::clone(&m);
        let handle = thread::spawn(move || {
            let mut num = m.lock().unwrap();
            *num += 1;
        });

        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("Result: {}", *m.lock().unwrap());
}
