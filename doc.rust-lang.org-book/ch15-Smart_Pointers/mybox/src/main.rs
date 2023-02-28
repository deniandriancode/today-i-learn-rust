use std::ops::{Deref, Drop};

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
	MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
	&self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
	println!("Dropping CustomSmartPointer with data: {}", self.data);
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(5);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    
    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    let _c = CustomSmartPointer {
	data: String::from("my stuff"),
    };
    let _d = CustomSmartPointer {
	data: String::from("other stuff"),
    };
    drop(_c);
    println!("CustomSmartPointer created!");
}

fn hello(msg: &str) { 
    println!("Hello, {msg}!");
}
