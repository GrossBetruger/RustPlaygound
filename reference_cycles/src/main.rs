
use List::{Cons, Nil};
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match *self {
            Cons(_, ref item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    let a = Rc::new(Cons(0, RefCell::new(Rc::new(Nil))));

    println!("initial rc count (a): {}", Rc::strong_count(&a));
    println!("a - next item: {:?}", a.tail().unwrap());

    let b = Rc::new(Cons(1, RefCell::new(Rc::clone(&a))));

    println!("rc count (a) after creating b: {}", Rc::strong_count(&a));
    println!("initial rc count (b): {}", Rc::strong_count(&b));
    println!("b - next item: {:?}", b.tail().unwrap());

    // link 'b' to itself - thus creating a cycle
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after creating the cycle: {}", Rc::strong_count(&b));
    println!("a rc count after creating the cycle: {}", Rc::strong_count(&a));

    // calling tail on 'a' will stackoverflow
    println!("a -> next: {:?}", a.tail());
}
