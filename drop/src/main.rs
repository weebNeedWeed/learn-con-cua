struct CustomSmartPointer {
    value: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Drop with value: {}", self.value);
    }
}

fn main() {
    let x = CustomSmartPointer {
        value: String::from("hello"),
    };

    drop(x);
    println!("end of the fn");
}
