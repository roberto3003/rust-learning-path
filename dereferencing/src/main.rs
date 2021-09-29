use std::ops::Deref;
use List::{Cons, Nil};

enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    let _list = {
        Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    };

    let x = 5;
    let y = &x;
    let z = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *z);

    let y = MyBox::new(x);
    assert_eq!(5, *y);

    println!("x = {}", *y);
    println!("x = {}", &*y); //implicit coercion

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    hello(&*m);
    hello(&(*m));
    hello(&(*m)[..]);
}
