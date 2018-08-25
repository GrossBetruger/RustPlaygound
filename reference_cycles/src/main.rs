use List::{Cons, Nil};
use std::rc::Rc;
use std::cell::RefCell;
use std::rc::Weak;

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

#[derive(Debug)]
struct Node {
    data: i32,
    children: RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Weak<Node>>
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
    //    println!("a -> next: {:?}", a.tail());

    // Avoiding reference cycles:

    let leaf = Rc::new(Node
        {
            parent: RefCell::new(Weak::new()),
            data: 3,
            children: RefCell::new(vec![])
        });

    let branch = Rc::new(Node
        {
            parent: RefCell::new(Weak::new()),
            data: 5,
            children: RefCell::new(vec![Rc::clone(&leaf)])
        });

    // leaf is connected to its parent with weak ref, so that it can be dropped independently from its parent

    println!();
    println!("leaf parent before: = {:?}", leaf.parent.borrow().upgrade());

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!();
    println!("leaf's parent after: {:?}", leaf.parent.borrow().upgrade()); // yay! no stack overflow
    println!();

    // leaf has two strong refs corresponding to two owners - self and parent(branch)
    println!("leaf strong reference count: {}", Rc::strong_count(&leaf));
    println!("leaf weak reference count: {}", Rc::weak_count(&leaf));
    println!();
    // parent has one strong ref (self) and one weak (child - leaf)
    println!("branch strong reference count: {}", Rc::strong_count(&branch));
    println!("branch weak reference count: {}", Rc::weak_count(&branch));
}
