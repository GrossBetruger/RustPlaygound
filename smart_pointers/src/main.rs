use std::ops::Deref;
use std::rc::Rc;


enum List {
    Cons(i32, Box<List>),
    Nil
}

use List::{Cons, Nil};


enum RCList {
    RcCons(i32, Rc<RCList>),
    RcNil
}

use RCList::{RcCons, RcNil};


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

fn hello(name: &str) {
    println!("hello, {}", name)
}

struct CustomSmartPointer {
    data: String
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("dropping SmartPointer with data: {}", self.data)
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


    let name = String::from("Nick");
    // calling function with the expected type &str
    hello(&name);

    let boxed_name = MyBox(String::from("Dave"));
    //  Deref coercion in action:
    // a reference to a box for a type T automatically becomes a reference to the original T
    hello(&boxed_name);
    // so we don't need to do this (dereference box, take slice, create & ref to slice):
    hello(&(*boxed_name)[..]);

    // pointers are dropped FILO so 'smart2' will be dropped first
    let smart1 = CustomSmartPointer{data: String::from("first smart pointer")};
    let smart2 = CustomSmartPointer{data: String::from("second smart pointer")};

    println!("smart pointers created!");

    let manual_drop = CustomSmartPointer{data: String::from("drop me gently")};
    drop(manual_drop); // using std::mem::drop to manually drop pointer before going out of scope
    println!("oops, you dropped something...");


    // Reference Counters:

    // two lists that point to a third list (conceptually both should be owners of the third)

    let a = Rc::new(RcCons(3, Rc::new(RcCons(10,
                Rc::new(RcNil)))));

    // b, and c point to a:
    let b = RcCons(3, Rc::clone(&a));
    let c = RcCons(2, Rc::clone(&a));


}
