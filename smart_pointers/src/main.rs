use std::ops::Deref;


enum List {
    Cons(i32, Box<List>),
    Nil
}

use List::{Cons, Nil};


struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T>{
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    // deref returns a & reference which the compiler than knows how to further dereference when
    // using the * operator
    fn deref(&self) -> &T {
        &self.0
    }
}

fn main() {
    let list = Cons(1,
        Box::new(Cons(1,
            Box::new(Cons(2,
                Box::new(Cons(3, Box::new(Nil))))))));


    let x = 5;
    let y = &x;
    assert_eq!(x, 5);
    assert_eq!(*y, 5);

    let x = 2;
    let y = &&x;

    assert_eq!(x, 2);
    assert_eq!(**y, 2);

    let x = 5;
    let y = Box::new(x);

    assert_eq!(x, 5);
    assert_eq!(*y, 5);

    let x = 3;
    let y = MyBox(x);

    assert_eq!(x, 3);
    assert_eq!(*y, 3);



}
